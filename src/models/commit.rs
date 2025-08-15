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
    pub breaking_change_flag: bool,
    pub title: String,
    pub body: Option<String>,
    pub breaking_change_body: bool,
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
            breaking_change_flag: false,
            title: String::new(),
            body: None,
            breaking_change_body: false,
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
            commit.breaking_change_flag = parsed.breaking_change_flag;
            commit.title = parsed.title;
            commit.body = parsed.body;
            commit.breaking_change_body = parsed.breaking_change_body;
        }
        
        commit
    }
    
    /// Check if this commit represents a major change
    pub fn is_major_change(&self) -> bool {
        self.breaking_change_flag || self.breaking_change_body || self.commit_type == "major"
    }
    
    /// Check if this commit represents a minor change
    pub fn is_minor_change(&self) -> bool {
        self.commit_type == "feat" || self.commit_type == "minor"
    }
    
    /// Check if this commit represents a patch change
    pub fn is_patch_change(&self) -> bool {
        !(self.is_major_change() || self.is_minor_change() || self.is_noop_change())
    }
    
    /// Check if this commit represents a no-op change
    pub fn is_noop_change(&self) -> bool {
        self.commit_type == "chore" || self.commit_type == "noop"
    }
}

/// Trait for commit message parsers.
///
/// This trait defines the interface for commit message parsers. Parsers are used to
/// determine the type of change represented by a commit message, which is then used
/// to calculate the next semantic version.
///
/// Implementations of this trait should be able to:
/// - Determine if a commit message represents a major change
/// - Determine if a commit message represents a minor change
/// - Determine if a commit message represents a no-op change
/// - Determine if a commit message represents a breaking change
/// - Parse a commit message into a structured Commit object
pub trait CommitParser {
    /// Parse a commit message and determine if it represents a major change.
    ///
    /// A major change is one that breaks backward compatibility, such as removing
    /// a public API or changing the behavior of an existing API in a way that
    /// requires users to update their code.
    fn is_major_change(&self, message: &str) -> bool;
    
    /// Parse a commit message and determine if it represents a minor change.
    ///
    /// A minor change is one that adds new functionality without breaking backward
    /// compatibility, such as adding a new API or extending an existing one.
    fn is_minor_change(&self, message: &str) -> bool;
    
    /// Parse a commit message and determine if it represents a no-op change.
    ///
    /// A no-op change is one that does not affect the functionality of the code,
    /// such as updating documentation, fixing typos, or refactoring code without
    /// changing its behavior.
    fn is_noop_change(&self, message: &str) -> bool;
    
    /// Parse a commit message and determine if it represents a breaking change.
    ///
    /// A breaking change is one that breaks backward compatibility, such as removing
    /// a public API or changing the behavior of an existing API in a way that
    /// requires users to update their code.
    fn is_breaking_change(&self, message: &str) -> bool;
    
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