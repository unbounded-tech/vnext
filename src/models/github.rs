//! GitHub-related data structures

use serde::{Deserialize, Serialize};

/// GitHub commit information
#[derive(Serialize, Deserialize, Debug)]
pub struct GitHubCommit {
    pub sha: String,
    pub commit: GitHubCommitDetails,
    pub author: Option<GitHubUser>,
}

/// GitHub commit details
#[derive(Serialize, Deserialize, Debug)]
pub struct GitHubCommitDetails {
    pub author: GitHubCommitAuthor,
    pub message: String,
}

/// GitHub commit author information
#[derive(Serialize, Deserialize, Debug)]
pub struct GitHubCommitAuthor {
    pub name: String,
    pub email: String,
}

/// GitHub user information
#[derive(Serialize, Deserialize, Debug)]
pub struct GitHubUser {
    pub login: String,
    pub html_url: String,
}