//! Core business logic of the application.
//!
//! This module contains the core business logic of the application,
//! organized by domain.

pub mod git;
pub mod github;
pub mod version;
pub mod changelog;

// Re-export commonly used functions
pub use git::{extract_repo_info, find_latest_tag, find_main_branch, open_repository, resolve_head, get_repo_info};
pub use github::enhance_with_github_info;
pub use version::{calculate_next_version, calculate_version_bump, parse_version, calculate_version};
pub use changelog::{output_result, output_fallback, format_changelog};