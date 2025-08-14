//! GitHub API integration

use crate::models::error::VNextError;
use crate::models::repo::RepoInfo;
use crate::models::commit::CommitAuthor;
use crate::models::github::{GitHubCommit};
use reqwest::blocking::Client;
use std::collections::HashMap;

/// Enhance commit summary with GitHub author information
pub fn enhance_with_github_info(
    repo_info: &RepoInfo,
    summary: &mut crate::models::commit::CommitSummary,
) -> Result<(), VNextError> {
    log::debug!("GitHub integration enabled, fetching commit author information");
    
    // Extract commit IDs from the summary
    let commit_ids: Vec<String> = summary.commits.iter()
        .map(|(id, _, _)| id.clone())
        .collect();
    
    // Fetch author information from GitHub API
    match fetch_commit_authors(&repo_info.owner, &repo_info.name, &commit_ids) {
        Ok(authors) => {
            log::debug!("Successfully fetched author information for {} commits", authors.len());
            
            // Create a map of commit IDs to authors
            let mut author_map = HashMap::new();
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
            Ok(())
        }
        Err(e) => {
            Err(VNextError::GithubError(format!("Failed to fetch author information: {}", e)))
        }
    }
}

/// Fetch commit author information from GitHub API
pub fn fetch_commit_authors(
    repo_owner: &str,
    repo_name: &str,
    commit_ids: &[String],
) -> Result<Vec<(String, Option<CommitAuthor>)>, VNextError> {
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
        
        let response = request.send()
            .map_err(|e| VNextError::GithubError(format!("Request failed: {}", e)))?;

        if response.status().is_success() {
            let commit: GitHubCommit = response.json()
                .map_err(|e| VNextError::GithubError(format!("Failed to parse response: {}", e)))?;
            
            let author = CommitAuthor {
                name: commit.commit.author.name,
                email: commit.commit.author.email,
                username: commit.author.map(|a| a.login),
            };
            
            results.push((commit_id.clone(), Some(author)));
        } else {
            log::debug!("Failed to fetch commit {} from GitHub API: {}", commit_id, response.status());
            log::debug!("This probably means that {} exists in your current repository but has not been pushed to the remote.", commit_id);
            results.push((commit_id.clone(), None));
        }
    }

    Ok(results)
}