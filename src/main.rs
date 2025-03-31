use clap::Parser;
use git2::{Repository, Commit};
use semver::{Version, Prerelease, BuildMetadata};

mod logging;

#[derive(Parser)]
#[clap(author, version, about = "Calculate the next version based on conventional commits")]
struct Cli {
    /// Perform a dry run without outputting the version (for testing)
    #[clap(long, action)]
    dry_run: bool,

    /// Output additional details about the version calculation
    #[clap(long, action)]
    verbose: bool,

    /// Specify a starting version instead of detecting from Git tags (e.g., "1.2.3" or "v1.2.3")
    #[clap(long)]
    start_version: Option<String>,
}

struct VersionBump {
    major: bool,
    minor: bool,
    patch: bool,
}

fn main() {
    logging::init_logging().expect("Failed to initialize logging");
    log::info!("Starting vnext...");

    let cli = Cli::parse();

    // Open the Git repository in the current directory, handle error gracefully
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => {
            log::error!("Failed to open Git repository: {}. This command must be run in a Git repository directory.", e);
            std::process::exit(1);
        }
    };

    // Get HEAD, handle case where it doesn’t exist (e.g., no commits)
    let head = match repo.head() {
        Ok(head_ref) => match head_ref.peel_to_commit() {
            Ok(commit) => commit,
            Err(e) => {
                log::error!("Failed to resolve HEAD to a commit: {}. The repository must have at least one commit.", e);
                std::process::exit(1);
            }
        },
        Err(e) => {
            log::error!("Failed to get HEAD: {}. The repository must have at least one commit on a branch (e.g., 'main' or 'master').", e);
            std::process::exit(1);
        }
    };

    // Determine the main branch (main or master)
    let main_branch = find_main_branch(&repo).expect("Failed to find main branch");
    log::debug!("Main branch detected: {}", main_branch);

    // Determine the starting version: either from CLI arg or latest tag
    let (start_version, last_tag_commit) = match &cli.start_version {
        Some(version_str) => {
            let version = parse_version(version_str).expect("Invalid start-version provided");
            if cli.verbose {
                log::info!("Using provided start version: {}", version);
            }
            (version, head.clone()) // Use HEAD as the commit if version is manual
        }
        None => {
            let (tag, commit) = find_latest_tag(&repo).unwrap_or_else(|| {
                log::info!("No previous release tags found, starting from 0.0.0");
                ("v0.0.0".to_string(), head.clone())
            });
            let version = parse_version(&tag).unwrap_or_else(|_| Version::new(0, 0, 0));
            if cli.verbose {
                log::info!("Last release: {} at commit {}", tag, commit.id());
            }
            (version, commit)
        }
    };

    // Find the merge base between the last tag (or HEAD if manual) and HEAD to account for release commits
    let merge_base = repo.merge_base(last_tag_commit.id(), head.id())
        .expect("Failed to find merge base");
    let merge_base_commit = repo.find_commit(merge_base).expect("Failed to find merge base commit");

    // Analyze commits from merge base to HEAD
    let bump = calculate_version_bump(&repo, &merge_base_commit, &head);
    
    // Calculate the next version
    let next_version = calculate_next_version(&start_version, &bump);
    if cli.verbose {
        log::info!("Version bump: major={}, minor={}, patch={}", bump.major, bump.minor, bump.patch);
        log::info!("Next version: {}", next_version);
    } else {
        log::info!("Next version: {}", next_version);
    }

    // Output the version unless dry-run is enabled
    if !cli.dry_run {
        println!("{}", next_version);
    }
}

fn find_main_branch(repo: &Repository) -> Option<String> {
    for branch in ["main", "master"] {
        if repo.find_branch(branch, git2::BranchType::Local).is_ok() {
            return Some(branch.to_string());
        }
    }
    None
}

fn find_latest_tag(repo: &Repository) -> Option<(String, Commit)> {
    let tags = repo.tag_names(None).expect("Failed to get tag names");
    let mut latest_version: Option<(String, Commit)> = None;
    let mut max_version = Version::new(0, 0, 0);

    for tag in tags.iter().flatten() {
        if let Ok(tag_ref) = repo.find_reference(&format!("refs/tags/{}", tag)) {
            if let Ok(commit) = tag_ref.peel_to_commit() {
                if let Ok(version) = parse_version(tag) {
                    if version > max_version {
                        max_version = version;
                        latest_version = Some((tag.to_string(), commit));
                    }
                }
            }
        }
    }
    latest_version
}

fn parse_version(tag: &str) -> Result<Version, semver::Error> {
    let cleaned_tag = tag.trim_start_matches('v'); // Remove 'v' prefix if present
    Version::parse(cleaned_tag)
}

fn calculate_version_bump(repo: &Repository, from: &Commit, to: &Commit) -> VersionBump {
    let mut revwalk = repo.revwalk().expect("Failed to create revwalk");
    revwalk.push(to.id()).expect("Failed to push HEAD to revwalk");
    revwalk.hide(from.id()).expect("Failed to hide merge base");

    let mut bump = VersionBump { major: false, minor: false, patch: false };

    for commit_id in revwalk {
        let commit = repo.find_commit(commit_id.expect("Invalid commit ID")).expect("Failed to find commit");
        let message = commit.message().unwrap_or("");

        log::debug!("Analyzing commit: {} - {}", commit.id(), message.lines().next().unwrap_or(""));

        if message.contains("BREAKING CHANGE") {
            bump.major = true;
        } else if message.starts_with("feat:") {
            bump.minor = true;
        } else if message.starts_with("fix:") {
            bump.patch = true;
        }
    }

    bump
}

fn calculate_next_version(current: &Version, bump: &VersionBump) -> Version {
    let mut next = current.clone();
    next.pre = Prerelease::EMPTY; // Clear prerelease
    next.build = BuildMetadata::EMPTY; // Clear build metadata

    if bump.major {
        next.major += 1;
        next.minor = 0;
        next.patch = 0;
    } else if bump.minor {
        next.minor += 1;
        next.patch = 0;
    } else if bump.patch {
        next.patch += 1;
    }

    next
}