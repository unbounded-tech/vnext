//! Git repository operations

use git2::{Commit, Repository};
use crate::models::error::VNextError;

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