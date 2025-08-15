//! Custom regex-based commit parser implementation

use crate::models::commit::{Commit, CommitParser};
use regex::Regex;

// Default regex patterns
pub const MAJOR_REGEX_STR: &str = r"(?m)^major(\(.+\))?:.*";
pub const MINOR_REGEX_STR: &str = r"(?m)^(minor|feat)(\(.+\))?:.*";
pub const NOOP_REGEX_STR: &str = r"(?m)^(noop|chore)(\(.+\))?:.*";
pub const BREAKING_REGEX_STR: &str = r"(?s)^[^\n]*\n\nBREAKING CHANGE:.*";

/// Parser using custom regex patterns for different types of changes
pub struct CustomRegexParser {
    major_regex: Regex,
    minor_regex: Regex,
    noop_regex: Regex,
    breaking_regex: Regex,
}

impl CustomRegexParser {
    pub fn new(
        major_pattern: &str,
        minor_pattern: &str,
        noop_pattern: &str,
        breaking_pattern: &str,
    ) -> Result<Self, regex::Error> {
        Ok(CustomRegexParser {
            major_regex: Regex::new(major_pattern)?,
            minor_regex: Regex::new(minor_pattern)?,
            noop_regex: Regex::new(noop_pattern)?,
            breaking_regex: Regex::new(breaking_pattern)?,
        })
    }
    
    pub fn default() -> Self {
        CustomRegexParser::new(
            MAJOR_REGEX_STR,
            MINOR_REGEX_STR,
            NOOP_REGEX_STR,
            BREAKING_REGEX_STR,
        ).expect("Default regex patterns should be valid")
    }
}

impl CommitParser for CustomRegexParser {
    fn is_major_change(&self, message: &str) -> bool {
        let is_breaking = self.breaking_regex.is_match(message);
        let is_major = self.major_regex.is_match(message);
        
        if is_breaking || is_major {
            log::debug!("Custom parser: Detected major change in commit: {}", message.lines().next().unwrap_or(""));
            if is_breaking {
                log::debug!("  Reason: Matches breaking change pattern");
            }
            if is_major {
                log::debug!("  Reason: Matches major change pattern");
            }
        }
        
        is_breaking || is_major
    }
    
    fn is_minor_change(&self, message: &str) -> bool {
        let is_minor = self.minor_regex.is_match(message);
        
        if is_minor {
            log::debug!("Custom parser: Detected minor change in commit: {}", message.lines().next().unwrap_or(""));
        }
        
        is_minor
    }
    
    fn is_noop_change(&self, message: &str) -> bool {
        let is_noop = self.noop_regex.is_match(message);
        
        if is_noop {
            log::debug!("Custom parser: Detected no-op change in commit: {}", message.lines().next().unwrap_or(""));
        }
        
        is_noop
    }
    
    fn is_breaking_change(&self, message: &str) -> bool {
        let is_breaking = self.breaking_regex.is_match(message);
        
        if is_breaking {
            log::debug!("Custom parser: Detected breaking change in commit: {}", message.lines().next().unwrap_or(""));
        }
        
        is_breaking
    }
    
    fn parse_commit(&self, commit_id: String, message: String) -> Commit {
        log::debug!("Customer Regex Parser - Message: {}", message);
        let mut commit = Commit::new(commit_id, message.clone());
        
        // Extract basic information from the message
        // This won't be as detailed as the conventional commit parser
        if let Some(first_line) = message.lines().next() {
            commit.title = first_line.to_string();
            
            // Try to extract commit type from the first line
            if let Some(colon_pos) = first_line.find(':') {
                let commit_type = first_line[..colon_pos].trim();
                commit.commit_type = commit_type.to_string();
                
                // Check for scope in parentheses
                if let (Some(open_paren), Some(close_paren)) = (commit_type.find('('), commit_type.find(')')) {
                    if open_paren < close_paren {
                        commit.scope = Some(commit_type[open_paren+1..close_paren].to_string());
                        commit.commit_type = commit_type[..open_paren].to_string();
                    }
                }
            }
        }
        
        // Extract body if present
        if let Some(body_start) = message.find("\n\n") {
            commit.body = Some(message[body_start+2..].trim().to_string());
        }
        
        // Set flags based on regex matches
        commit.breaking_change_flag = self.is_breaking_change(&message);
        
        commit
    }
    
    fn name(&self) -> &str {
        "custom-regex"
    }
}