//! A library for calculating the next version based on conventional commits.

pub mod cli;
pub mod constants;
pub mod git;
pub mod github;
pub mod logging;
pub mod version;

// Re-export commonly used types and functions
pub use cli::Cli;
pub use constants::{BREAKING_REGEX_STR, MAJOR_REGEX_STR, MINOR_REGEX_STR, NOOP_REGEX_STR};
pub use git::{calculate_version_bump, extract_repo_info, find_latest_tag, find_main_branch};
pub use version::{calculate_next_version, parse_version, CommitSummary, VersionBump};