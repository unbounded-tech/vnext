//! Custom regex-based commit parser implementation

use crate::models::commit::{Commit, CommitParser};
use regex::Regex;

// Default regex patterns
pub const COMMIT_TYPE_REGEX_STR: &str = r"^([\w-]+)(?:.*)?!?:.*";
pub const TITLE_REGEX_STR: &str = r"^[\w-]+(?:.*)?!?:\s(.*)";
pub const BODY_REGEX_STR: &str = r"^[\w-]+(?:.*)?!?:\s.*\n\s*(?:BREAKING CHANGE:)?\s*([\s\S]*)";
pub const SCOPE_REGEX_STR: &str = r"^[\w-]+(?:\((.*)\))?!?:.*";
pub const BREAKING_REGEX_STR: &str = r"(?:^[^\n]*\n\nBREAKING CHANGE:.*|^[\w-]+(?:.*)?!:.*)";
// Regex for extracting scope from commit message

/// Parser using custom regex patterns for commit parts
pub struct CustomRegexParser {
    commit_type_regex: Regex,
    title_regex: Regex,
    body_regex: Regex,
    breaking_regex: Regex,
    scope_regex: Regex,
}

impl CustomRegexParser {
    pub fn new(
        commit_type_pattern: &str,
        title_pattern: &str,
        body_pattern: &str,
        breaking_pattern: &str,
        scope_pattern: &str,
    ) -> Result<Self, regex::Error> {
        Ok(CustomRegexParser {
            commit_type_regex: Regex::new(commit_type_pattern)?,
            title_regex: Regex::new(title_pattern)?,
            body_regex: Regex::new(body_pattern)?,
            breaking_regex: Regex::new(breaking_pattern)?,
            scope_regex: Regex::new(scope_pattern)?,
        })
    }
    
    pub fn default() -> Self {
        CustomRegexParser::new(
            COMMIT_TYPE_REGEX_STR,
            TITLE_REGEX_STR,
            BODY_REGEX_STR,
            BREAKING_REGEX_STR,
            SCOPE_REGEX_STR,
        ).expect("Default regex patterns should be valid")
    }
}

impl CommitParser for CustomRegexParser {
    fn parse_commit(&self, commit_id: String, message: String) -> Commit {
        log::debug!("Customer Regex Parser - Message: {}", message);
        let mut commit = Commit::new(commit_id, message.clone());
                
        // Extract commit title using title_regex
        if let Some(captures) = self.title_regex.captures(&message) {
            if let Some(title_match) = captures.get(1) {
                commit.title = title_match.as_str().to_string();
            }
        }
        
        // Extract commit type using commit_type_regex
        if let Some(captures) = self.commit_type_regex.captures(&message) {
            if let Some(type_match) = captures.get(1) {
                commit.commit_type = type_match.as_str().to_string();
            }
        }
        
        // Extract scope using scope_regex
        if let Some(captures) = self.scope_regex.captures(&message) {
            if let Some(scope_match) = captures.get(1) {
                commit.scope = Some(scope_match.as_str().to_string());
            }
        }
        
        // Extract title using title_regex
        if let Some(captures) = self.title_regex.captures(&message) {
            if let Some(title_match) = captures.get(1) {
                commit.title = title_match.as_str().to_string();
            }
        }
        
        
        // Extract body using body_regex
        if let Some(captures) = self.body_regex.captures(&message) {
            if let Some(body_match) = captures.get(1) {
                commit.body = Some(body_match.as_str().trim().to_string());
            }
        }
        
        // Set breaking change flag based on regex match
        commit.has_breaking_change = self.breaking_regex.is_match(&message);
        
        // Log information about the commit for debugging
        log::debug!("Custom parser: Parsed commit: {}", message.lines().next().unwrap_or(""));
        log::debug!("  Type: {}", commit.commit_type);
        if let Some(scope) = &commit.scope {
            log::debug!("  Scope: {}", scope);
        }
        log::debug!("  Title: {}", commit.title);
        log::debug!("  Breaking change: {}", commit.has_breaking_change);
        
        commit
    }
    
    fn name(&self) -> &str {
        "custom-regex"
    }
}