//! CLI interface definition

use clap::{Parser, Subcommand};
use crate::commands;
use crate::models::error::VNextError;
use crate::parsers::custom::{MAJOR_REGEX_STR, MINOR_REGEX_STR, NOOP_REGEX_STR, BREAKING_REGEX_STR, TYPE_REGEX_STR, SCOPE_REGEX_STR};

/// CLI for calculating the next version based on conventional commits
#[derive(Parser, Debug)]
#[clap(author, version, about = "Calculate the next version based on conventional commits")]
pub struct Cli {
    /// Parser strategy to use (conventional or custom)
    #[clap(long, default_value = "conventional")]
    pub parser: String,

    /// Regex for commits triggering a major version bump (used with custom parser)
    #[clap(long, default_value = MAJOR_REGEX_STR)]
    pub major: String,

    /// Regex for commits triggering a minor version bump (used with custom parser)
    #[clap(long, default_value = MINOR_REGEX_STR)]
    pub minor: String,

    /// Regex for commits that should not trigger a version bump (used with custom parser)
    #[clap(long, default_value = NOOP_REGEX_STR)]
    pub noop: String,

    /// Regex for commits indicating a breaking change (used with custom parser)
    #[clap(long, default_value = BREAKING_REGEX_STR)]
    pub breaking: String,

    /// Regex for extracting commit type from message (used with custom parser)
    #[clap(long, default_value = TYPE_REGEX_STR)]
    pub type_pattern: String,

    /// Regex for extracting commit scope from message (used with custom parser)
    #[clap(long, default_value = SCOPE_REGEX_STR)]
    pub scope_pattern: String,

    /// Comma-separated list of commit types that trigger a major version bump
    #[clap(long, default_value = "major")]
    pub major_commit_types: String,

    /// Comma-separated list of commit types that trigger a minor version bump
    #[clap(long, default_value = "feat,minor")]
    pub minor_commit_types: String,

    /// Comma-separated list of commit types that should not trigger a version bump
    #[clap(long, default_value = "chore,noop")]
    pub noop_commit_types: String,

    /// Output the changelog with the next version
    #[clap(long)]
    pub changelog: bool,

    /// Disable header scaling in changelog (by default, h1->h4, h2->h5, h3->h6)
    #[clap(long)]
    pub no_header_scaling: bool,

    /// Output the current version that vnext is bumping from
    #[clap(long)]
    pub current: bool,

    /// Subcommands
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

/// CLI subcommands
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate a deploy key for GitHub repository
    GenerateDeployKey {
        /// GitHub repository owner (e.g., unbounded-tech)
        #[clap(long)]
        owner: Option<String>,
        
        /// GitHub repository name
        #[clap(long)]
        name: Option<String>,
        
        /// Name of the deploy key
        #[clap(long, default_value = "DEPLOY_KEY")]
        key_name: Option<String>,
        
        /// Overwrite existing deploy key and secret if they exist
        #[clap(long)]
        overwrite: bool,
    },
}

/// Parse command line arguments
pub fn parse_cli() -> Cli {
    Cli::parse()
}

/// Run the CLI
pub fn run(cli: Cli) -> Result<(), VNextError> {
    // Check if a subcommand was provided
    if let Some(command) = &cli.command {
        match command {
            Commands::GenerateDeployKey { owner, name, key_name, overwrite } => {
                return commands::deploy_key::generate_deploy_key(owner.clone(), name.clone(), key_name.clone(), *overwrite);
            }
        }
    }
    
    // If no subcommand was provided, run the default vnext calculation logic
    commands::vnext::run_vnext_command(
        &cli.parser,
        &cli.major,
        &cli.minor,
        &cli.noop,
        &cli.breaking,
        &cli.type_pattern,
        &cli.scope_pattern,
        &cli.major_commit_types,
        &cli.minor_commit_types,
        &cli.noop_commit_types,
        cli.changelog,
        cli.no_header_scaling,
        cli.current,
    )
}