//! Commit-related data structures

/// Represents a commit author
#[derive(Clone, Debug)]
pub struct CommitAuthor {
    pub name: String,
    #[allow(dead_code)]
    pub email: String,
    pub username: Option<String>,
}

/// Represents a parsed conventional commit message
#[derive(Clone, Debug)]
pub struct Commit {
    pub commit_id: String,
    pub raw_message: String,
    pub commit_type: String,
    pub scope: Option<String>,
    pub has_breaking_change: bool,  // Single flag for breaking changes
    pub title: String,
    pub body: Option<String>,
    pub author: Option<CommitAuthor>,
}

impl Commit {
    /// Create a new Commit instance with minimal information
    pub fn new(commit_id: String, raw_message: String) -> Self {
        Commit {
            commit_id,
            raw_message,
            commit_type: String::new(),
            scope: None,
            has_breaking_change: false,
            title: String::new(),
            body: None,
            author: None,
        }
    }
    
    /// Parse a commit message using the conventional commit format
    pub fn parse(commit_id: String, message: String) -> Self {
        let mut commit = Commit::new(commit_id, message.clone());
        
        // Use the master regex to parse the message
        if let Some(parsed) = crate::parsers::conventional::parse_conventional_commit(&message) {
            commit.commit_type = parsed.commit_type;
            commit.scope = parsed.scope;
            // Set has_breaking_change if either flag or body indicates a breaking change
            commit.has_breaking_change = parsed.breaking_change_flag || parsed.breaking_change_body;
            commit.title = parsed.title;
            commit.body = parsed.body;
        }
        
        commit
    }
    
    /// Check if this commit represents a major change
    pub fn is_major_change(&self, major_types: &[&str]) -> bool {
        self.has_breaking_change || major_types.contains(&self.commit_type.as_str())
    }
    
    /// Check if this commit represents a minor change
    pub fn is_minor_change(&self, minor_types: &[&str]) -> bool {
        minor_types.contains(&self.commit_type.as_str())
    }
    
    /// Check if this commit represents a patch change
    pub fn is_patch_change(&self, major_types: &[&str], minor_types: &[&str], noop_types: &[&str]) -> bool {
        !(self.is_major_change(major_types) || self.is_minor_change(minor_types) || self.is_noop_change(noop_types))
    }
    
    /// Check if this commit represents a no-op change
    pub fn is_noop_change(&self, noop_types: &[&str]) -> bool {
        noop_types.contains(&self.commit_type.as_str())
    }
}

/// Trait for commit message parsers.
///
/// This trait defines the interface for commit message parsers. Parsers are used to
/// parse commit messages into structured Commit objects, which are then used to
/// determine the type of change represented by the commit.
///
/// Implementations of this trait should be able to:
/// - Parse a commit message into a structured Commit object
/// - Provide a name for identification
pub trait CommitParser {
    /// Parse a commit message into a structured Commit object.
    ///
    /// This method parses a commit message into a structured Commit object, which
    /// contains information about the commit such as its type, scope, title, body,
    /// and whether it represents a breaking change.
    fn parse_commit(&self, commit_id: String, message: String) -> Commit;
    
    /// Get the name of the parser.
    ///
    /// This method returns a string that identifies the parser, such as "conventional"
    /// or "custom-regex".
    fn name(&self) -> &str;
}