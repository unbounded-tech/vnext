//! Repository information data structures

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