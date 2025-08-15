//! Regex pattern compilation and validation

use log::error;
pub use regex::Regex;
use std::process;

use crate::cli;

// Constant regex string literals used for defaults.
pub const MAJOR_REGEX_STR: &str = r"(?m)^major(\(.+\))?:.*";
pub const MINOR_REGEX_STR: &str = r"(?m)^(minor|feat)(\(.+\))?:.*";
pub const NOOP_REGEX_STR: &str = r"(?m)^(noop|chore)(\(.+\))?:.*";
pub const BREAKING_REGEX_STR: &str = r"(?s)^[^\n]*\n\nBREAKING CHANGE:.*";

// New regex for parsing conventional commits
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

/// Compile and validate regex patterns from CLI arguments
///
/// Returns a tuple of compiled regexes for major, minor, noop, and breaking changes
pub fn compile_regexes(cli: &cli::Cli) -> (Regex, Regex, Regex, Regex) {
    let major_re = Regex::new(&cli.major).unwrap_or_else(|e| {
        error!("Invalid major regex '{}': {}", cli.major, e);
        process::exit(1);
    });
    let minor_re = Regex::new(&cli.minor).unwrap_or_else(|e| {
        error!("Invalid minor regex '{}': {}", cli.minor, e);
        process::exit(1);
    });
    let noop_re = Regex::new(&cli.noop).unwrap_or_else(|e| {
        error!("Invalid noop regex '{}': {}", cli.noop, e);
        process::exit(1);
    });
    let breaking_re = Regex::new(&cli.breaking).unwrap_or_else(|e| {
        error!("Invalid breaking regex '{}': {}", cli.breaking, e);
        process::exit(1);
    });
    
    (major_re, minor_re, noop_re, breaking_re)
}