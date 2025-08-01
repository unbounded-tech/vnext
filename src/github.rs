use crate::version::CommitAuthor;
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
            log::warn!("Failed to fetch commit {} from GitHub API: {}", commit_id, response.status());
            results.push((commit_id.clone(), None));
        }
    }

    Ok(results)
}

/// Extract repository owner and name from a git remote URL
pub fn extract_repo_info(remote_url: &str) -> Option<(String, String)> {
    // Handle SSH URLs like git@github.com:owner/repo.git
    if remote_url.starts_with("git@github.com:") {
        let path = remote_url.trim_start_matches("git@github.com:");
        let path = path.trim_end_matches(".git");
        let parts: Vec<&str> = path.split('/').collect();
        if parts.len() >= 2 {
            return Some((parts[0].to_string(), parts[1].to_string()));
        }
    }
    
    // Handle HTTPS URLs like https://github.com/owner/repo.git
    if remote_url.contains("github.com") {
        let url = url::Url::parse(remote_url).ok()?;
        let path = url.path().trim_start_matches('/').trim_end_matches(".git");
        let parts: Vec<&str> = path.split('/').collect();
        if parts.len() >= 2 {
            return Some((parts[0].to_string(), parts[1].to_string()));
        }
    }
    
    None
}