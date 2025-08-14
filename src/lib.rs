//! A library for calculating the next version based on conventional commits.

pub mod models;
pub mod utils;
pub mod services;
pub mod commands;
pub mod cli;

// Re-export commonly used types and functions
pub use cli::Cli;
pub use models::error::VNextError;
pub use models::version::VersionBump;
pub use models::commit::{CommitSummary, CommitAuthor};
pub use models::repo::RepoInfo;
pub use services::git::{extract_repo_info, find_latest_tag, find_main_branch, open_repository, resolve_head, get_repo_info};
pub use services::github::enhance_with_github_info;
pub use services::version::{calculate_next_version, calculate_version_bump, parse_version, calculate_version};
pub use services::changelog::{output_result, output_fallback};
pub use utils::regex::{compile_regexes, BREAKING_REGEX_STR, MAJOR_REGEX_STR, MINOR_REGEX_STR, NOOP_REGEX_STR};

// Re-export for backward compatibility with tests
pub mod version {
    pub use crate::models::version::VersionBump;
    pub use crate::models::commit::{CommitSummary, CommitAuthor};
    pub use crate::services::version::{calculate_next_version, calculate_version_bump, parse_version, calculate_version, find_version_base};
}

pub mod git {
    pub use crate::services::git::{extract_repo_info, find_latest_tag, find_main_branch, open_repository, resolve_head};
}

pub mod changelog {
    pub use crate::models::repo::RepoInfo;
    pub use crate::services::git::get_repo_info;
    pub use crate::services::changelog::{output_result, output_fallback};
}

pub mod github {
    pub use crate::services::github::enhance_with_github_info;
}

pub mod error {
    pub use crate::models::error::VNextError;
}