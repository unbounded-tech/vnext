use git2::Repository;

mod cli;
mod git;
mod github;
mod logging;
mod regex;
mod version;
mod vnext;

fn main() {
    logging::init_logging().expect("Failed to initialize logging");
    log::debug!("Starting vnext...");

    let cli = cli::parse_cli();

    log::debug!("Major bump regex: {}", cli.major);
    log::debug!("Minor bump regex: {}", cli.minor);
    log::debug!("No-op regex: {}", cli.noop);
    log::debug!("Breaking change regex: {}", cli.breaking);

    let (major_re, minor_re, noop_re, breaking_re) = regex::compile_regexes(&cli);

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

    let (start_version, base_commit) = vnext::find_version_base(&repo, &head);

    let (bump, mut summary) = git::calculate_version_bump(&repo, &base_commit, &head, &major_re, &minor_re, &noop_re, &breaking_re);

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

    // Get repository information
    let mut owner = String::new();
    let mut name = String::new();
    let mut is_github_repo = false;
    let mut is_gitlab_repo = false;
    let mut is_bitbucket_repo = false;
    
    // Check repository host
    if let Ok(remote) = repo.find_remote("origin") {
        if let Some(url) = remote.url() {
            if let Some((host, repo_owner, repo_name)) = git::extract_repo_info(url) {
                owner = repo_owner;
                name = repo_name;
                
                if host == "github.com" {
                    is_github_repo = true;
                    log::debug!("Detected GitHub repository: {}/{}", owner, name);
                } else if host == "gitlab.com" {
                    is_gitlab_repo = true;
                    log::debug!("Detected GitLab repository: {}/{}", owner, name);
                } else if host == "bitbucket.org" {
                    is_bitbucket_repo = true;
                    log::debug!("Detected BitBucket repository: {}/{}", owner, name);
                } else {
                    log::debug!("Detected repository at {}: {}/{}", host, owner, name);
                }
            }
        }
    }
    
    // Auto-enable GitHub flag if detection is enabled and repository is on GitHub
    let use_github = cli.github || is_github_repo;
    
    // Define flags for GitLab and BitBucket (for future implementation)
    let use_gitlab = is_gitlab_repo;
    let use_bitbucket = is_bitbucket_repo;
    
    // Handle changelog generation with repository-specific integrations
    if cli.changelog {
        if use_github {
            log::debug!("GitHub integration enabled, fetching commit author information");
            
            // Extract commit IDs from the summary
            let commit_ids: Vec<String> = summary.commits.iter()
                .map(|(id, _, _)| id.clone())
                .collect();
            
            // Fetch author information from GitHub API
            match github::fetch_commit_authors(&owner, &name, &commit_ids) {
                Ok(authors) => {
                    log::debug!("Successfully fetched author information for {} commits", authors.len());
                    
                    // Create a map of commit IDs to authors
                    let mut author_map = std::collections::HashMap::new();
                    for (commit_id, author) in authors {
                        author_map.insert(commit_id, author);
                    }
                    
                    // Update the summary with author information
                    for i in 0..summary.commits.len() {
                        let commit_id = &summary.commits[i].0;
                        if let Some(author) = author_map.get(commit_id) {
                            if let Some(author_info) = author {
                                log::debug!("Adding author information for commit {}: {}", commit_id, author_info.name);
                                summary.commits[i].2 = Some(author_info.clone());
                            }
                        }
                    }
                }
                Err(e) => {
                    log::warn!("Failed to fetch author information from GitHub API: {}", e);
                }
            }
        } else if use_gitlab {
            log::debug!("GitLab repository detected, but GitLab integration is not implemented yet");
        } else if use_bitbucket {
            log::debug!("BitBucket repository detected, but BitBucket integration is not implemented yet");
        }
    }

    if cli.changelog {
        println!("{}", summary.format_changelog(&next_version));
    } else {
        println!("{}", next_version);
    }
}


