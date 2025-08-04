use git2::Repository;
use crate::git;
use crate::version::CommitSummary;
use semver::Version;
use std::string::String;

/// Repository information structure
pub struct RepoInfo {
    pub owner: String,
    pub name: String,
    pub is_github_repo: bool,
    pub is_gitlab_repo: bool,
    pub is_bitbucket_repo: bool,
}

impl RepoInfo {
    /// Create a new empty RepoInfo
    pub fn new() -> Self {
        RepoInfo {
            owner: String::new(),
            name: String::new(),
            is_github_repo: false,
            is_gitlab_repo: false,
            is_bitbucket_repo: false,
        }
    }
}

/// Get repository information from a git repository
/// 
/// This function extracts the owner, name, and repository type from a git repository.
/// It uses the `extract_repo_info` function from the `git` module to extract the
/// repository information from the remote URL.
/// 
/// # Arguments
/// 
/// * `repo` - A reference to a git repository
/// 
/// # Returns
/// 
/// A `RepoInfo` struct containing the repository information
pub fn get_repo_info(repo: &Repository) -> RepoInfo {
    let mut repo_info = RepoInfo::new();
    
    // Check repository host
    if let Ok(remote) = repo.find_remote("origin") {
        if let Some(url) = remote.url() {
            if let Some((host, repo_owner, repo_name)) = git::extract_repo_info(url) {
                repo_info.owner = repo_owner;
                repo_info.name = repo_name;
                
                if host == "github.com" {
                    repo_info.is_github_repo = true;
                    log::debug!("Detected GitHub repository: {}/{}", repo_info.owner, repo_info.name);
                } else if host == "gitlab.com" {
                    repo_info.is_gitlab_repo = true;
                    log::debug!("Detected GitLab repository: {}/{}", repo_info.owner, repo_info.name);
                } else if host == "bitbucket.org" {
                    repo_info.is_bitbucket_repo = true;
                    log::debug!("Detected BitBucket repository: {}/{}", repo_info.owner, repo_info.name);
                } else {
                    log::debug!("Detected repository at {}: {}/{}", host, repo_info.owner, repo_info.name);
                }
            }
        }
    }
    
    repo_info
}

/// Output the result of the version calculation
pub fn output_result(
    next_version: &Version,
    summary: &CommitSummary,
    show_changelog: bool,
    no_header_scaling: bool,
) {
    if show_changelog {
        println!("{}", summary.format_changelog(next_version, no_header_scaling));
    } else {
        println!("{}", next_version);
    }
}

/// Output a fallback result when an error occurs
pub fn output_fallback(show_changelog: bool) {
    if show_changelog {
        println!("## What's changed in 0.0.0\n\n* No changes\n\n---");
    } else {
        println!("0.0.0");
    }
}