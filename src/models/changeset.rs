//! Changeset-related data structures

use crate::models::commit::CommitAuthor;

/// Represents a summary of changes for version calculation
pub struct ChangesetSummary {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub noop: u32,
    pub commits: Vec<(String, String, Option<CommitAuthor>)>, // (commit_id, message, author)
}

impl ChangesetSummary {
    pub fn new() -> Self {
        ChangesetSummary {
            major: 0,
            minor: 0,
            patch: 0,
            noop: 0,
            commits: Vec::new(),
        }
    }

    // The format_changelog method has been moved to the changelog service
}