//! Commit-related data structures

// No imports needed after removing format_changelog method

/// Represents a commit author
#[derive(Clone, Debug)]
pub struct CommitAuthor {
    pub name: String,
    #[allow(dead_code)]
    pub email: String,
    pub username: Option<String>,
}

/// Represents a summary of commits for version calculation
pub struct CommitSummary {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub noop: u32,
    pub commits: Vec<(String, String, Option<CommitAuthor>)>, // (commit_id, message, author)
}

impl CommitSummary {
    pub fn new() -> Self {
        CommitSummary {
            major: 0,
            minor: 0,
            patch: 0,
            noop: 0,
            commits: Vec::new(),
        }
    }

    // The format_changelog method has been moved to the changelog service
}