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
pub mod changeset;

// Re-export commonly used types
pub use version::VersionBump;
pub use commit::{Commit, CommitAuthor};
pub use changeset::ChangesetSummary;
pub use error::VNextError;
pub use repo::RepoInfo;