//! Regex pattern compilation and validation for conventional commits

use crate::models::commit::{Commit, CommitParser};
pub use regex::Regex;

// Regex for parsing conventional commits
pub const CONVENTIONAL_COMMIT_REGEX_STR: &str = r"^([\w-]+)(?:\(([^\)]+)\))?(!)?:\s*(.*)\n*((BREAKING CHANGE:)?\s?([\s\S]*))?";

/// Represents the parsed components of a conventional commit message
#[derive(Clone, Debug)]
pub struct ParsedCommit {
    pub commit_type: String,
    pub scope: Option<String>,
    pub breaking_change_flag: bool,
    pub title: String,
    pub body: Option<String>,
    pub breaking_change_body: bool,
}

/// Parse a conventional commit message into its components
pub fn parse_conventional_commit(message: &str) -> Option<ParsedCommit> {
    log::debug!("Conventional Commit Parser - Message: {}", message);
    // Master regex for the entire commit message including header and body
    // Format: type(scope)?!?: title\n*(BREAKING CHANGE:)?\s?([\s\S]*)
    let commit_regex = Regex::new(CONVENTIONAL_COMMIT_REGEX_STR).ok()?;
    
    // Parse using the regex
    let captures = commit_regex.captures(message)?;
    
    let commit_type = captures.get(1)?.as_str().to_string();
    let scope = captures.get(2).map(|m| m.as_str().to_string());
    let breaking_change_flag = captures.get(3).is_some();
    let title = captures.get(4)?.as_str().to_string();
    
    // Get body from capture group 7 (if it exists)
    let body = captures.get(7).map(|m| {
        let body_str = m.as_str().trim_start();
        if body_str.is_empty() {
            None
        } else {
            Some(body_str.to_string())
        }
    }).flatten();
    
    // Check for breaking change in body using capture group 6
    let breaking_change_body = captures.get(6).is_some();
    
    Some(ParsedCommit {
        commit_type,
        scope,
        breaking_change_flag,
        title,
        body,
        breaking_change_body,
    })
}

/// Parser for Conventional Commits
pub struct ConventionalCommitParser;

impl ConventionalCommitParser {
    pub fn new() -> Self {
        ConventionalCommitParser
    }
}

impl CommitParser for ConventionalCommitParser {
    fn parse_commit(&self, commit_id: String, message: String) -> Commit {
        let mut commit = Commit::new(commit_id, message.clone());
        
        if let Some(parsed) = parse_conventional_commit(&message) {
            commit.commit_type = parsed.commit_type;
            commit.scope = parsed.scope;
            // Set has_breaking_change if either flag or body indicates a breaking change
            commit.has_breaking_change = parsed.breaking_change_flag || parsed.breaking_change_body;
            commit.title = parsed.title;
            commit.body = parsed.body;
        } else {
            log::debug!("Conventional parser: Could not parse commit message: {}", message.lines().next().unwrap_or(""));
        }
        
        commit
    }
    
    fn name(&self) -> &str {
        "conventional"
    }
}