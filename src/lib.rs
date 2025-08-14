//! A library for calculating the next version based on conventional commits.

pub mod changelog;
pub mod cli;
pub mod error;
pub mod git;
pub mod github;
pub mod logging;
pub mod regex;
pub mod version;

// Re-export commonly used types and functions
pub use changelog::{get_repo_info, output_result, output_fallback};
pub use cli::Cli;
pub use error::VNextError;
pub use git::{extract_repo_info, find_latest_tag, find_main_branch, open_repository, resolve_head};
pub use github::enhance_with_github_info;
pub use regex::{compile_regexes, BREAKING_REGEX_STR, MAJOR_REGEX_STR, MINOR_REGEX_STR, NOOP_REGEX_STR};
pub use version::{calculate_next_version, calculate_version_bump, parse_version, CommitSummary, VersionBump, find_version_base, calculate_version};