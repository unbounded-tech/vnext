//! Git repository operations

use git2::{Commit, Repository};
use crate::models::error::VNextError;
use crate::models::repo::RepoInfo;
use url::Url;

/// Find the main branch ("main" or "master").
pub fn find_main_branch(repo: &Repository) -> Option<String> {
    for branch in ["main", "master"] {
        if repo.find_branch(branch, git2::BranchType::Local).is_ok() {
            return Some(branch.to_string());
        }
    }
    None
}

/// Find the latest semver tag in the repo, returning (tag_name, commit).
pub fn find_latest_tag(repo: &Repository) -> Option<(String, Commit)> {
    let tags = repo.tag_names(None).expect("Failed to get tag names");
    let mut latest: Option<(String, Commit)> = None;
    let mut max_version = crate::services::version::parse_version("0.0.0").unwrap();

    for tag in tags.iter().flatten() {
        if let Ok(reference) = repo.find_reference(&format!("refs/tags/{}", tag)) {
            if let Ok(commit) = reference.peel_to_commit() {
                if let Ok(version) = crate::services::version::parse_version(tag) {
                    if version > max_version {
                        max_version = version;
                        latest = Some((tag.to_string(), commit));
                    }
                }
            }
        }
    }
    latest
}

/// Open the Git repository in the current directory
pub fn open_repository() -> Result<Repository, VNextError> {
    Repository::open(".").map_err(|e| e.into())
}

/// Resolve the HEAD reference to a commit
pub fn resolve_head(repo: &Repository) -> Result<Commit, VNextError> {
    let head_ref = repo.head()?;
    let commit = head_ref.peel_to_commit()?;
    Ok(commit)
}

/// Extract repository information from a git remote URL
/// Returns (host, owner, name) if successful
pub fn extract_repo_info(remote_url: &str) -> Option<(String, String, String)> {
    // Handle SSH URLs like git@github.com:owner/repo.git or git@gitlab.com:owner/repo.git
    if remote_url.starts_with("git@") && remote_url.contains(':') {
        let host_part = remote_url.split('@').nth(1)?.split(':').next()?;
        let path = remote_url.split(':').nth(1)?;
        let path = path.trim_end_matches(".git");
        let parts: Vec<&str> = path.split('/').collect();
        if parts.len() >= 2 {
            return Some((host_part.to_string(), parts[0].to_string(), parts[1].to_string()));
        }
    }
    
    // Handle HTTPS URLs like https://github.com/owner/repo.git or https://gitlab.com/owner/repo.git
    if let Ok(url) = Url::parse(remote_url) {
        let host = url.host_str()?;
        let path = url.path().trim_start_matches('/').trim_end_matches(".git");
        let parts: Vec<&str> = path.split('/').collect();
        if parts.len() >= 2 {
            return Some((host.to_string(), parts[0].to_string(), parts[1].to_string()));
        }
    }
    
    None
}

/// Get repository information from a git repository
///
/// This function extracts the owner, name, and repository type from a git repository.
/// It uses the `extract_repo_info` function to extract the repository information
/// from the remote URL.
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
            if let Some((host, repo_owner, repo_name)) = extract_repo_info(url) {
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