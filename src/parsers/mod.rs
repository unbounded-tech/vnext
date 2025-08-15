//! Parsers for various commit message formats and standards.
//!
//! This module contains parsers for different commit message formats and standards used
//! throughout the application. The parsers are used to determine the type of change
//! represented by a commit message, which is then used to calculate the next semantic version.
//!
//! The parser system is designed to be flexible and extensible, allowing for different
//! parsing strategies to be used. The default strategy is to use the conventional commit
//! format, but custom regex patterns can also be used.
//!
//! # Parser Strategies
//!
//! ## Conventional Commits
//!
//! The conventional commit parser follows the [Conventional Commits](https://www.conventionalcommits.org/)
//! specification. It parses commit messages in the format:
//!
//! ```text
//! <type>[optional scope]: <description>
//!
//! [optional body]
//!
//! [optional footer(s)]
//! ```
//!
//! Where:
//! - `type` is one of: feat, fix, docs, style, refactor, perf, test, chore, etc.
//! - `scope` is an optional identifier for the section of the codebase affected
//! - `description` is a short description of the change
//! - `body` is an optional detailed description of the change
//! - `footer` is an optional section for breaking changes or references to issues
//!
//! ## Custom Regex
//!
//! The custom regex parser uses regular expressions to determine the type of change
//! represented by a commit message. It uses four regex patterns:
//!
//! - `major_pattern`: Matches commits that trigger a major version bump
//! - `minor_pattern`: Matches commits that trigger a minor version bump
//! - `noop_pattern`: Matches commits that should not trigger a version bump
//! - `breaking_pattern`: Matches commits that indicate a breaking change
//!
//! # Usage
//!
//! ```no_run
//! use vnext::parsers::{ParserFactory, ParserStrategy};
//! use vnext::models::commit::CommitParser;
//!
//! // Create a conventional commit parser
//! let parser = ParserFactory::create(&ParserStrategy::Conventional);
//!
//! // Or create a custom regex parser
//! let custom_parser = ParserFactory::create(&ParserStrategy::CustomRegex {
//!     major_pattern: r"(?m)^major(\(.+\))?:.*".to_string(),
//!     minor_pattern: r"(?m)^(minor|feat)(\(.+\))?:.*".to_string(),
//!     noop_pattern: r"(?m)^(noop|chore)(\(.+\))?:.*".to_string(),
//!     breaking_pattern: r"(?s)^[^\n]*\n\nBREAKING CHANGE:.*".to_string(),
//! });
//!
//! // Use the parser to determine the type of change
//! let is_major = parser.is_major_change("feat!: Breaking change");
//! let is_minor = parser.is_minor_change("feat: Add new feature");
//! let is_noop = parser.is_noop_change("chore: Update dependencies");
//! ```

// Conventional commit parser
pub mod conventional;

// Custom regex parser
pub mod custom;

// Parser factory
pub mod factory;

// Re-export commonly used functions and types
pub use conventional::{parse_conventional_commit, ParsedCommit, CONVENTIONAL_COMMIT_REGEX_STR, ConventionalCommitParser};
pub use custom::{CustomRegexParser, MAJOR_REGEX_STR, MINOR_REGEX_STR, NOOP_REGEX_STR, BREAKING_REGEX_STR, TYPE_REGEX_STR, SCOPE_REGEX_STR};
pub use factory::{ParserFactory, ParserStrategy};