//! Commit-related data structures

/// Represents a commit author
#[derive(Clone, Debug)]
pub struct CommitAuthor {
    pub name: String,
    #[allow(dead_code)]
    pub email: String,
    pub username: Option<String>,
}