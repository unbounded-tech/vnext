//! Regex pattern compilation and validation

pub use regex::Regex;

// Regex for parsing conventional commits
pub const CONVENTIONAL_COMMIT_REGEX_STR: &str = r"^([\w-]+)(?:\(([^\)]+)\))?(!)?:\s*(.*)";

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
    // Master regex for the first line (header)
    // Format: type(scope)?!?: title
    let header_regex = Regex::new(CONVENTIONAL_COMMIT_REGEX_STR).ok()?;
    
    // Split message into header and body
    let mut lines = message.lines();
    let header = lines.next()?;
    
    // Parse header
    let captures = header_regex.captures(header)?;
    
    let commit_type = captures.get(1)?.as_str().to_string();
    let scope = captures.get(2).map(|m| m.as_str().to_string());
    let breaking_change_flag = captures.get(3).is_some();
    let title = captures.get(4)?.as_str().to_string();
    
    // Parse body
    let body_text: Vec<&str> = lines.collect();
    let body = if !body_text.is_empty() {
        // Join the body lines and trim leading newlines
        let body_str = body_text.join("\n");
        let trimmed_body = body_str.trim_start();
        Some(trimmed_body.to_string())
    } else {
        None
    };
    
    // Check for breaking change in body
    // According to the spec, BREAKING CHANGE: must be at the start of the first line of the body
    let breaking_change_body = if let Some(ref body) = body {
        body.starts_with("BREAKING CHANGE:")
    } else {
        false
    };
    
    Some(ParsedCommit {
        commit_type,
        scope,
        breaking_change_flag,
        title,
        body,
        breaking_change_body,
    })
}
