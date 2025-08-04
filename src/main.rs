use regex::Regex;

mod changelog;
mod cli;
mod deploy_key;
mod error;
mod git;
mod github;
mod logging;
mod regex;
mod version;
mod vnext;

use crate::error::VNextError;

/// Initialize the CLI: set up logging, parse CLI arguments, and compile regex patterns
fn initialize_cli() -> Result<(cli::Cli, (Regex, Regex, Regex, Regex)), VNextError> {
    // Initialize logging
    logging::init_logging().map_err(|e| VNextError::Other(format!("Failed to initialize logging: {}", e)))?;
    log::debug!("Starting vnext...");

    // Parse CLI arguments
    let cli = cli::parse_cli();

    // Log debug information about regex patterns
    log::debug!("Major bump regex: {}", cli.major);
    log::debug!("Minor bump regex: {}", cli.minor);
    log::debug!("No-op regex: {}", cli.noop);
    log::debug!("Breaking change regex: {}", cli.breaking);

    // Compile regex patterns
    let regexes = regex::compile_regexes(&cli);
    
    Ok((cli, regexes))
}

/// Main application logic
fn run() -> Result<(), VNextError> {
    // Initialize the application
    let (cli, regexes) = initialize_cli()?;
    
    // Check if a subcommand was provided
    if let Some(command) = &cli.command {
        match command {
            cli::Commands::GenerateDeployKey { owner, name, key_name } => {
                return deploy_key::generate_deploy_key(owner.clone(), name.clone(), key_name.clone());
            }
        }
    }
    
    // If no subcommand was provided, run the default version calculation logic
    let (major_re, minor_re, noop_re, breaking_re) = regexes;

    // Open repository and handle errors
    let repo = match git::open_repository() {
        Ok(repo) => repo,
        Err(e) => {
            log::debug!("No Git repository found: {}. Assuming version 0.0.0.", e);
            changelog::output_fallback(cli.changelog);
            return Ok(());
        }
    };

    // Resolve HEAD and handle errors
    let head = match git::resolve_head(&repo) {
        Ok(head) => head,
        Err(e) => {
            log::debug!("Failed to resolve HEAD: {}. Assuming version 0.0.0.", e);
            changelog::output_fallback(cli.changelog);
            return Ok(());
        }
    };
    log::debug!("HEAD commit: {}", head.id());

    // Calculate version
    let (next_version, mut summary) = match vnext::calculate_version(&repo, &head, &major_re, &minor_re, &noop_re, &breaking_re) {
        Ok(result) => result,
        Err(e) => {
            log::error!("Failed to calculate version: {}", e);
            changelog::output_fallback(cli.changelog);
            return Ok(());
        }
    };
    
    // Get repository information
    let repo_info = changelog::get_repo_info(&repo);
    
    // Use GitHub integration if repository is on GitHub
    let use_github = repo_info.is_github_repo;
    
    // Handle GitHub integration if needed
    if cli.changelog && use_github {
        if let Err(e) = github::enhance_with_github_info(&repo_info, &mut summary) {
            log::warn!("Failed to fetch author information from GitHub API: {}", e);
        }
    }
    
    // Output result
    changelog::output_result(&next_version, &summary, cli.changelog, cli.no_header_scaling);
    
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        log::error!("{}", e);
        std::process::exit(1);
    }
}

