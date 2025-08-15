//! A library for calculating the next version based on conventional commits.

pub mod models;
pub mod utils;
pub mod core;
pub mod commands;
pub mod cli;
pub mod parsers;

// Re-export commonly used types and functions
pub use cli::Cli;
pub use models::error::VNextError;
pub use models::version::VersionBump;
pub use models::commit::{Commit, CommitAuthor};
pub use models::changeset::ChangesetSummary;
pub use models::repo::RepoInfo;
pub use core::git::{extract_repo_info, find_latest_tag, find_trunk_branch, open_repository, resolve_head, get_repo_info};
pub use core::github::enhance_with_github_info;
pub use core::version::{calculate_next_version, calculate_version_bump, parse_version, calculate_version};
pub use core::changelog::{output_result, output_fallback, format_changelog};
pub use parsers::conventional::{parse_conventional_commit, CONVENTIONAL_COMMIT_REGEX_STR};

// Re-export for backward compatibility with tests
pub mod version {
    pub use crate::models::version::VersionBump;
    pub use crate::models::commit::{Commit, CommitAuthor};
    pub use crate::models::changeset::ChangesetSummary;
    pub use crate::core::version::{calculate_next_version, calculate_version_bump, parse_version, calculate_version, find_version_base};
}

pub mod git {
    pub use crate::core::git::{extract_repo_info, find_latest_tag, find_trunk_branch, open_repository, resolve_head};
}

pub mod changelog {
    pub use crate::models::repo::RepoInfo;
    pub use crate::core::git::get_repo_info;
    pub use crate::core::changelog::{output_result, output_fallback, format_changelog};
}

pub mod github {
    pub use crate::core::github::enhance_with_github_info;
}

pub mod error {
    pub use crate::models::error::VNextError;
}