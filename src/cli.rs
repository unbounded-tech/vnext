use clap::Parser;
use crate::constants::{MAJOR_REGEX_STR, MINOR_REGEX_STR, NOOP_REGEX_STR, BREAKING_REGEX_STR};

#[derive(Parser)]
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
}

pub fn parse_cli() -> Cli {
    Cli::parse()
}