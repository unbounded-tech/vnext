//! Parsers for various formats and standards.
//!
//! This module contains parsers for different formats and standards used
//! throughout the application.

// Conventional commit parser
pub mod conventional;

// Re-export commonly used functions and types
pub use conventional::{parse_conventional_commit, ParsedCommit, CONVENTIONAL_COMMIT_REGEX_STR};