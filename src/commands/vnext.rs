//! VNext command implementation

use crate::models::error::VNextError;
use crate::core::git;
use crate::core::version;
use crate::core::changelog;
use crate::parsers::{ParserFactory, ParserStrategy};

/// Run the vnext command
pub fn run_vnext_command(
    parser_name: &str,
    breaking_pattern: &str,
    type_pattern: &str,
    title_pattern: &str,
    body_pattern: &str,
    scope_pattern: &str,
    major_commit_types: &str,
    minor_commit_types: &str,
    noop_commit_types: &str,
    show_changelog: bool,
    no_header_scaling: bool,
    current: bool,
) -> Result<(), VNextError> {
    // Parse comma-separated commit types
    let major_types: Vec<&str> = major_commit_types.split(',').map(|s| s.trim()).collect();
    let minor_types: Vec<&str> = minor_commit_types.split(',').map(|s| s.trim()).collect();
    let noop_types: Vec<&str> = noop_commit_types.split(',').map(|s| s.trim()).collect();
    
    log::debug!("Using commit types:");
    log::debug!("  Major types: {:?}", major_types);
    log::debug!("  Minor types: {:?}", minor_types);
    log::debug!("  No-op types: {:?}", noop_types);
    
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
                commit_type_pattern: type_pattern.to_string(),
                title_pattern: title_pattern.to_string(),
                body_pattern: body_pattern.to_string(),
                breaking_pattern: breaking_pattern.to_string(),
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
        &repo, &head, &current_version, &base_commit, &*parser,
        &major_types, &minor_types, &noop_types
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