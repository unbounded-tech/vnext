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
    fn is_major_change(&self, message: &str) -> bool {
        if let Some(parsed) = parse_conventional_commit(message) {
            let is_major = parsed.breaking_change_flag || parsed.breaking_change_body || parsed.commit_type == "major";
            
            if is_major {
                log::debug!("Conventional parser: Detected major change in commit: {}", message.lines().next().unwrap_or(""));
                if parsed.breaking_change_flag {
                    log::debug!("  Reason: Breaking change flag (!) present");
                }
                if parsed.breaking_change_body {
                    log::debug!("  Reason: BREAKING CHANGE: in commit body");
                }
                if parsed.commit_type == "major" {
                    log::debug!("  Reason: Commit type is 'major'");
                }
            }
            
            is_major
        } else {
            log::debug!("Conventional parser: Could not parse commit message: {}", message.lines().next().unwrap_or(""));
            false
        }
    }
    
    fn is_minor_change(&self, message: &str) -> bool {
        if let Some(parsed) = parse_conventional_commit(message) {
            let is_minor = parsed.commit_type == "feat" || parsed.commit_type == "minor";
            
            if is_minor {
                log::debug!("Conventional parser: Detected minor change in commit: {}", message.lines().next().unwrap_or(""));
                log::debug!("  Reason: Commit type is '{}'", parsed.commit_type);
            }
            
            is_minor
        } else {
            false
        }
    }
    
    fn is_noop_change(&self, message: &str) -> bool {
        if let Some(parsed) = parse_conventional_commit(message) {
            let is_noop = parsed.commit_type == "chore" || parsed.commit_type == "noop";
            
            if is_noop {
                log::debug!("Conventional parser: Detected no-op change in commit: {}", message.lines().next().unwrap_or(""));
                log::debug!("  Reason: Commit type is '{}'", parsed.commit_type);
            }
            
            is_noop
        } else {
            false
        }
    }
    
    fn is_breaking_change(&self, message: &str) -> bool {
        if let Some(parsed) = parse_conventional_commit(message) {
            let is_breaking = parsed.breaking_change_flag || parsed.breaking_change_body;
            
            if is_breaking {
                log::debug!("Conventional parser: Detected breaking change in commit: {}", message.lines().next().unwrap_or(""));
                if parsed.breaking_change_flag {
                    log::debug!("  Reason: Breaking change flag (!) present");
                }
                if parsed.breaking_change_body {
                    log::debug!("  Reason: BREAKING CHANGE: in commit body");
                }
            }
            
            is_breaking
        } else {
            false
        }
    }
    
    fn parse_commit(&self, commit_id: String, message: String) -> Commit {
        let mut commit = Commit::new(commit_id, message.clone());
        
        if let Some(parsed) = parse_conventional_commit(&message) {
            commit.commit_type = parsed.commit_type;
            commit.scope = parsed.scope;
            commit.breaking_change_flag = parsed.breaking_change_flag;
            commit.title = parsed.title;
            commit.body = parsed.body;
            commit.breaking_change_body = parsed.breaking_change_body;
        }
        
        commit
    }
    
    fn name(&self) -> &str {
        "conventional"
    }
}