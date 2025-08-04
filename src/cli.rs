use clap::{Parser, Subcommand};
use crate::regex::{MAJOR_REGEX_STR, MINOR_REGEX_STR, NOOP_REGEX_STR, BREAKING_REGEX_STR};

#[derive(Parser, Debug)]
#[clap(author, version, about = "Calculate the next version based on conventional commits")]
pub struct Cli {
    /// Regex for commits triggering a major version bump
    #[clap(long, default_value = MAJOR_REGEX_STR)]
    pub major: String,

    /// Regex for commits triggering a minor version bump
    #[clap(long, default_value = MINOR_REGEX_STR)]
    pub minor: String,

    /// Regex for commits that should not trigger a version bump
    #[clap(long, default_value = NOOP_REGEX_STR)]
    pub noop: String,

    /// Regex for commits indicating a breaking change
    #[clap(long, default_value = BREAKING_REGEX_STR)]
    pub breaking: String,

    /// Output the changelog with the next version
    #[clap(long)]
    pub changelog: bool,

    /// Disable header scaling in changelog (by default, h1->h4, h2->h5, h3->h6)
    #[clap(long)]
    pub no_header_scaling: bool,

    /// Subcommands
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

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
    },
}

pub fn parse_cli() -> Cli {
    Cli::parse()
}