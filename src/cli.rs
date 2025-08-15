//! CLI interface definition

use clap::{Parser, Subcommand};
use crate::commands;
use crate::models::error::VNextError;

/// CLI for calculating the next version based on conventional commits
#[derive(Parser, Debug)]
#[clap(author, version, about = "Calculate the next version based on conventional commits")]
pub struct Cli {

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
        cli.changelog,
        cli.no_header_scaling,
        cli.current,
    )
}