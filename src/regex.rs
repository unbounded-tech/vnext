//! Regex pattern compilation and validation

use log::error;
use regex::Regex;
use std::process;

use crate::cli;

// Constant regex string literals used for defaults.
pub const MAJOR_REGEX_STR: &str = r"(?m)^major(\(.+\))?:.*";
pub const MINOR_REGEX_STR: &str = r"(?m)^(minor|feat)(\(.+\))?:.*";
pub const NOOP_REGEX_STR: &str = r"(?m)^(noop|chore)(\(.+\))?:.*";
pub const BREAKING_REGEX_STR: &str = r"(?s)^[^\n]*\n\nBREAKING CHANGE:.*";

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