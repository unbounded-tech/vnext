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

/// Trait for commit message parsers
pub trait CommitParser {
    /// Parse a commit message into a Commit struct
    fn parse(&self, commit_id: String, message: String) -> Commit;
}

/// Conventional commit parser implementation
pub struct ConventionalCommitParser;

impl CommitParser for ConventionalCommitParser {
    fn parse(&self, commit_id: String, message: String) -> Commit {
        Commit::parse(commit_id, message)
    }
}