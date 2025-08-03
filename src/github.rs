use crate::version::CommitAuthor;
use crate::git;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct GitHubCommit {
    sha: String,
    commit: GitHubCommitDetails,
    author: Option<GitHubUser>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GitHubCommitDetails {
    author: GitHubCommitAuthor,
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct GitHubCommitAuthor {
    name: String,
    email: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct GitHubUser {
    login: String,
    html_url: String,
}

/// Fetch commit author information from GitHub API
pub fn fetch_commit_authors(
    repo_owner: &str,
    repo_name: &str,
    commit_ids: &[String],
) -> Result<Vec<(String, Option<CommitAuthor>)>, Box<dyn Error>> {
    let client = Client::new();
    let mut results = Vec::new();

    for commit_id in commit_ids {
        let url = format!(
            "https://api.github.com/repos/{}/{}/commits/{}",
            repo_owner, repo_name, commit_id
        );

        // Check for GITHUB_TOKEN environment variable
        let mut request = client
            .get(&url)
            .header("User-Agent", "vnext-cli");
            
        // Add authorization header if GITHUB_TOKEN is available
        if let Ok(token) = std::env::var("GITHUB_TOKEN") {
            log::debug!("Using GITHUB_TOKEN for authentication");
            request = request.header("Authorization", format!("token {}", token));
        }
        
        let response = request.send()?;

        if response.status().is_success() {
            let commit: GitHubCommit = response.json()?;
            
            let author = CommitAuthor {
                name: commit.commit.author.name,
                email: commit.commit.author.email,
                username: commit.author.map(|a| a.login),
            };
            
            results.push((commit_id.clone(), Some(author)));
        } else {
            log::debug!("Failed to fetch commit {} from GitHub API: {}", commit_id, response.status());
            log::debug!("The probably means that {} exists in your current repository but has not been pushed to the remote.", commit_id);
            results.push((commit_id.clone(), None));
        }
    }

    Ok(results)
}

/// Check if a repository is hosted on GitHub and extract owner and name
pub fn is_github_repo(remote_url: &str) -> Option<(String, String)> {
    if let Some((host, owner, name)) = git::extract_repo_info(remote_url) {
        // Check if the host is github.com
        if host == "github.com" {
            return Some((owner, name));
        }
    }
    
    None
}