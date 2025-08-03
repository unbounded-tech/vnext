//! A library for calculating the next version based on conventional commits.

pub mod changelog;
pub mod cli;
pub mod git;
pub mod github;
pub mod logging;
pub mod regex;
pub mod version;
pub mod vnext;

// Re-export commonly used types and functions
pub use changelog::get_repo_info;
pub use cli::Cli;
pub use git::{calculate_version_bump, extract_repo_info, find_latest_tag, find_main_branch};
pub use regex::{compile_regexes, BREAKING_REGEX_STR, MAJOR_REGEX_STR, MINOR_REGEX_STR, NOOP_REGEX_STR};
pub use version::{calculate_next_version, parse_version, CommitSummary, VersionBump};
pub use vnext::find_version_base;