//! VNext command implementation

use crate::models::error::VNextError;
use crate::core::git;
use crate::core::version;
use crate::core::changelog;
use crate::parsers::{ParserFactory, ParserStrategy};

/// Run the vnext command
pub fn run_vnext_command(
    parser_name: &str,
    major_pattern: &str,
    minor_pattern: &str,
    noop_pattern: &str,
    breaking_pattern: &str,
    type_pattern: &str,
    scope_pattern: &str,
    show_changelog: bool,
    no_header_scaling: bool,
    current: bool,
) -> Result<(), VNextError> {
    // Create the appropriate parser based on the strategy
    log::debug!("Using parser strategy: {}", parser_name);
    
    let strategy = match parser_name {
        "conventional" => {
            log::debug!("Selected conventional commit parser strategy");
            ParserStrategy::Conventional
        },
        "custom" => {
            log::debug!("Selected custom regex parser strategy");
            ParserStrategy::CustomRegex {
                major_pattern: major_pattern.to_string(),
                minor_pattern: minor_pattern.to_string(),
                noop_pattern: noop_pattern.to_string(),
                breaking_pattern: breaking_pattern.to_string(),
                type_pattern: type_pattern.to_string(),
                scope_pattern: scope_pattern.to_string(),
            }
        },
        _ => {
            log::warn!("Unknown parser strategy '{}', falling back to conventional", parser_name);
            ParserStrategy::Conventional
        }
    };
    
    let parser = ParserFactory::create(&strategy);
    log::debug!("Parser initialized: {}", parser.name());

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
        &repo, &head, &current_version, &base_commit, &*parser
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
        if let Err(e) = crate::core::github::enhance_with_github_info(&repo_info, &mut summary) {
            log::warn!("Failed to fetch author information from GitHub API: {}", e);
        }
    }
    
    // Output result
    changelog::output_result(&next_version, &summary, show_changelog, no_header_scaling, &current_version, &repo_info);
    
    Ok(())
}