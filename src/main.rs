use git2::Repository;
use regex::Regex;

mod cli;
mod constants;
mod git;
mod logging;
mod version;

fn main() {
    logging::init_logging().expect("Failed to initialize logging");
    log::debug!("Starting vnext...");

    let cli = cli::parse_cli();

    log::debug!("Major bump regex: {}", cli.major);
    log::debug!("Minor bump regex: {}", cli.minor);
    log::debug!("No-op regex: {}", cli.noop);
    log::debug!("Breaking change regex: {}", cli.breaking);

    let major_re = Regex::new(&cli.major).unwrap_or_else(|e| {
        log::error!("Invalid major regex '{}': {}", cli.major, e);
        std::process::exit(1);
    });
    let minor_re = Regex::new(&cli.minor).unwrap_or_else(|e| {
        log::error!("Invalid minor regex '{}': {}", cli.minor, e);
        std::process::exit(1);
    });
    let noop_re = Regex::new(&cli.noop).unwrap_or_else(|e| {
        log::error!("Invalid noop regex '{}': {}", cli.noop, e);
        std::process::exit(1);
    });
    let breaking_re = Regex::new(&cli.breaking).unwrap_or_else(|e| {
        log::error!("Invalid breaking regex '{}': {}", cli.breaking, e);
        std::process::exit(1);
    });

    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => {
            log::debug!("No Git repository found: {}. Assuming version 0.0.0.", e);
            if cli.changelog {
                println!("## What's changed in 0.0.0\n\n* No changes\n\n---");
            } else {
                println!("0.0.0");
            }
            std::process::exit(0);
        }
    };

    let head = match repo.head() {
        Ok(head_ref) => match head_ref.peel_to_commit() {
            Ok(commit) => commit,
            Err(e) => {
                log::debug!(
                    "Failed to resolve HEAD to a commit: {}. Assuming version 0.0.0.",
                    e
                );
                if cli.changelog {
                    println!("## What's changed in 0.0.0\n\n* No changes\n\n---");
                } else {
                    println!("0.0.0");
                }
                std::process::exit(0);
            }
        },
        Err(e) => {
            log::debug!(
                "Failed to get HEAD: {}. Assuming version 0.0.0.",
                e
            );
            if cli.changelog {
                println!("## What's changed in 0.0.0\n\n* No changes\n\n---");
            } else {
                println!("0.0.0");
            }
            std::process::exit(0);
        }
    };
    log::debug!("HEAD commit: {}", head.id());

    let main_branch = git::find_main_branch(&repo).expect("Failed to find main branch");
    log::debug!("Main branch detected: {}", main_branch);

    let (start_version, last_tag_commit) = match git::find_latest_tag(&repo) {
        Some((tag, commit)) => {
            let version = version::parse_version(&tag).unwrap_or_else(|_| semver::Version::new(0, 0, 0));
            log::debug!("Last release: {} at commit {}", tag, commit.id());
            (version, commit)
        }
        None => {
            log::debug!("No previous release tags found, starting from 0.0.0");
            let version = semver::Version::new(0, 0, 0);
            
            // Find the initial commit in the repository
            let mut current = head.clone();
            let mut initial_commit = current.clone();
            
            // Traverse to the root commit by following the first parent chain
            loop {
                let parents = current.parents();
                if parents.count() == 0 {
                    // We've reached a commit with no parents (the initial commit)
                    initial_commit = current;
                    break;
                }
                
                // Move to the first parent and continue
                current = current.parents().next().unwrap();
            }
            
            log::debug!("Found initial commit: {}", initial_commit.id());
            (version, initial_commit)
        }
    };
    log::debug!("Last tag or base commit: {}", last_tag_commit.id());

    // Determine the base commit: use merge base with main if tag exists, otherwise use the initial commit
    let base_commit = if git::find_latest_tag(&repo).is_some() {
        let merge_base = repo
            .merge_base(head.id(), last_tag_commit.id())
            .expect("Failed to find merge base between HEAD and tag");
        repo.find_commit(merge_base)
            .expect("Failed to find merge base commit")
    } else {
        // When no tags exist, we want to analyze all commits from the initial commit to HEAD
        last_tag_commit.clone()
    };
    log::debug!("Base commit for analysis: {}", base_commit.id());

    let (bump, summary) = git::calculate_version_bump(&repo, &base_commit, &head, &major_re, &minor_re, &noop_re, &breaking_re);

    log::debug!(
        "Commits pending release: {} major, {} minor, {} patch, {} noop",
        summary.major, summary.minor, summary.patch, summary.noop
    );

    let next_version = version::calculate_next_version(&start_version, &bump);
    log::debug!(
        "Version bump: major={}, minor={}, patch={}",
        bump.major, bump.minor, bump.patch
    );
    log::debug!("Next version: {}", next_version);

    if cli.changelog {
        println!("{}", summary.format_changelog(&next_version));
    } else {
        println!("{}", next_version);
    }
}
