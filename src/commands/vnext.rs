//! VNext command implementation

use crate::models::error::VNextError;
use crate::services::git;
use crate::services::version;
use crate::services::changelog;
use crate::utils::regex;
use regex::Regex;

/// Run the vnext command
pub fn run_vnext_command(
    major_pattern: &str,
    minor_pattern: &str,
    noop_pattern: &str,
    breaking_pattern: &str,
    show_changelog: bool,
    no_header_scaling: bool,
    current: bool,
) -> Result<(), VNextError> {
    // Compile regex patterns
    let major_re = Regex::new(major_pattern).map_err(|e| VNextError::RegexError(e))?;
    let minor_re = Regex::new(minor_pattern).map_err(|e| VNextError::RegexError(e))?;
    let noop_re = Regex::new(noop_pattern).map_err(|e| VNextError::RegexError(e))?;
    let breaking_re = Regex::new(breaking_pattern).map_err(|e| VNextError::RegexError(e))?;

    // Open repository and handle errors
    let repo = match git::open_repository() {
        Ok(repo) => repo,
        Err(e) => {
            log::debug!("No Git repository found: {}. Assuming version 0.0.0.", e);
            changelog::output_fallback(show_changelog);
            return Ok(());
        }
    };

    // Resolve HEAD and handle errors
    let head = match git::resolve_head(&repo) {
        Ok(head) => head,
        Err(e) => {
            log::debug!("Failed to resolve HEAD: {}. Assuming version 0.0.0.", e);
            changelog::output_fallback(show_changelog);
            return Ok(());
        }
    };
    log::debug!("HEAD commit: {}", head.id());

    // If --current flag is set, output the current version and return early
    let (current_version, base_commit) = version::find_version_base(&repo, &head);
    if current {
        println!("{}", current_version);
        return Ok(());
    }

    // Calculate version
    let (next_version, mut summary) = match version::calculate_version(
        &repo, &head, &major_re, &minor_re, &noop_re, &breaking_re, &current_version, &base_commit
    ) {
        Ok(result) => result,
        Err(e) => {
            log::error!("Failed to calculate version: {}", e);
            changelog::output_fallback(show_changelog);
            return Ok(());
        }
    };
    
    // Get repository information
    let repo_info = git::get_repo_info(&repo);
    
    // Use GitHub integration if repository is on GitHub
    let use_github = repo_info.is_github_repo;
    
    // Handle GitHub integration if needed
    if show_changelog && use_github {
        if let Err(e) = crate::services::github::enhance_with_github_info(&repo_info, &mut summary) {
            log::warn!("Failed to fetch author information from GitHub API: {}", e);
        }
    }
    
    // Output result
    changelog::output_result(&next_version, &summary, show_changelog, no_header_scaling, &current_version, &repo_info);
    
    Ok(())
}