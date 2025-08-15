//! Custom regex-based commit parser implementation

use crate::models::commit::{Commit, CommitParser};
use regex::Regex;

// Default regex patterns
pub const MAJOR_REGEX_STR: &str = r"(?m)^major(\(.+\))?:.*";
pub const MINOR_REGEX_STR: &str = r"(?m)^(minor|feat)(\(.+\))?:.*";
pub const NOOP_REGEX_STR: &str = r"(?m)^(noop|chore)(\(.+\))?:.*";
pub const BREAKING_REGEX_STR: &str = r"(?s)^[^\n]*\n\nBREAKING CHANGE:.*";
// Regex for extracting type and scope from commit message
pub const TYPE_REGEX_STR: &str = r"^([\w-]+)((.*))?:";
pub const SCOPE_REGEX_STR: &str = r"^[\w-]+\((.*)\)!?:";

/// Parser using custom regex patterns for different types of changes
pub struct CustomRegexParser {
    major_regex: Regex,
    minor_regex: Regex,
    noop_regex: Regex,
    breaking_regex: Regex,
    type_regex: Regex,
    scope_regex: Regex,
}

impl CustomRegexParser {
    pub fn new(
        major_pattern: &str,
        minor_pattern: &str,
        noop_pattern: &str,
        breaking_pattern: &str,
        type_pattern: &str,
        scope_pattern: &str,
    ) -> Result<Self, regex::Error> {
        Ok(CustomRegexParser {
            major_regex: Regex::new(major_pattern)?,
            minor_regex: Regex::new(minor_pattern)?,
            noop_regex: Regex::new(noop_pattern)?,
            breaking_regex: Regex::new(breaking_pattern)?,
            type_regex: Regex::new(type_pattern)?,
            scope_regex: Regex::new(scope_pattern)?,
        })
    }
    
    pub fn default() -> Self {
        CustomRegexParser::new(
            MAJOR_REGEX_STR,
            MINOR_REGEX_STR,
            NOOP_REGEX_STR,
            BREAKING_REGEX_STR,
            TYPE_REGEX_STR,
            SCOPE_REGEX_STR,
        ).expect("Default regex patterns should be valid")
    }
}

impl CommitParser for CustomRegexParser {
    fn parse_commit(&self, commit_id: String, message: String) -> Commit {
        log::debug!("Customer Regex Parser - Message: {}", message);
        let mut commit = Commit::new(commit_id, message.clone());
        
        // Extract basic information from the message
        // This won't be as detailed as the conventional commit parser
        if let Some(first_line) = message.lines().next() {
            commit.title = first_line.to_string();
            
            // Extract commit type using type_regex
            if let Some(captures) = self.type_regex.captures(first_line) {
                if let Some(type_match) = captures.get(1) {
                    commit.commit_type = type_match.as_str().to_string();
                }
            }
            
            // Extract scope using scope_regex
            if let Some(captures) = self.scope_regex.captures(first_line) {
                if let Some(scope_match) = captures.get(1) {
                    commit.scope = Some(scope_match.as_str().to_string());
                }
            }
            
            // Extract title (everything after the colon and space)
            if let Some(colon_pos) = first_line.find(": ") {
                if colon_pos + 2 < first_line.len() {
                    commit.title = first_line[colon_pos + 2..].to_string();
                }
            }
        }
        
        // Extract body if present
        if let Some(body_start) = message.find("\n\n") {
            commit.body = Some(message[body_start+2..].trim().to_string());
        }
        
        // Set flags based on regex matches
        let is_breaking = self.breaking_regex.is_match(&message);
        let is_major = self.major_regex.is_match(&message);
        let is_minor = self.minor_regex.is_match(&message);
        let is_noop = self.noop_regex.is_match(&message);
        
        // Set breaking change flag
        commit.breaking_change_flag = is_breaking;
        
        // Handle major changes - if the message matches the major pattern,
        // ensure the commit type is set to "major" so is_major_change() returns true
        if is_major {
            commit.commit_type = "major".to_string();
        }
        
        // Log information about the commit type for debugging
        if is_breaking || is_major {
            log::debug!("Custom parser: Detected major change in commit: {}", message.lines().next().unwrap_or(""));
            if is_breaking {
                log::debug!("  Reason: Matches breaking change pattern");
            }
            if is_major {
                log::debug!("  Reason: Matches major change pattern");
            }
        } else if is_minor {
            log::debug!("Custom parser: Detected minor change in commit: {}", message.lines().next().unwrap_or(""));
        } else if is_noop {
            log::debug!("Custom parser: Detected no-op change in commit: {}", message.lines().next().unwrap_or(""));
        }
        
        commit
    }
    
    fn name(&self) -> &str {
        "custom-regex"
    }
}