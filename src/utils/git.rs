//! Git utility functions

use url::Url;

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