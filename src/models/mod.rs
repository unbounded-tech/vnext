//! Data structures used throughout the application.
//!
//! This module contains all the data structures and types used by the application,
//! organized by domain.

pub mod version;
pub mod commit;
pub mod error;
pub mod repo;
pub mod github;
pub mod deploy_key;

// Re-export commonly used types
pub use version::VersionBump;
pub use commit::{CommitSummary, CommitAuthor};
pub use error::VNextError;
pub use repo::RepoInfo;