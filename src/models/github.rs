//! GitHub-related data structures

use serde::{Deserialize, Serialize};

/// GitHub commit information
#[derive(Serialize, Deserialize, Debug)]
pub struct GitHubCommit {
    pub sha: String,
    pub commit: GitHubCommitDetails,
    pub author: Option<GitHubAccountInfo>,
}

/// GitHub commit details
#[derive(Serialize, Deserialize, Debug)]
pub struct GitHubCommitDetails {
    pub author: GitCommitAuthor,
    pub message: String,
}

/// Git commit author information (name and email from git commit)
#[derive(Serialize, Deserialize, Debug)]
pub struct GitCommitAuthor {
    pub name: String,
    pub email: String,
}

/// GitHub account information for the commit author
#[derive(Serialize, Deserialize, Debug)]
pub struct GitHubAccountInfo {
    pub login: String,
    // html_url field removed as it's not used anywhere in the codebase
}