//! Factory for creating commit parsers based on the selected strategy

use crate::models::commit::CommitParser;
use crate::parsers::conventional::ConventionalCommitParser;
use crate::parsers::custom::CustomRegexParser;

/// Enum representing different commit parsing strategies.
///
/// This enum defines the different strategies that can be used to parse commit messages.
/// The default strategy is to use the conventional commit format, but custom regex
/// patterns can also be used.
#[derive(Clone, Debug)]
pub enum ParserStrategy {
    /// Conventional Commits (https://www.conventionalcommits.org/).
    ///
    /// This strategy follows the Conventional Commits specification, which defines
    /// a structured format for commit messages.
    Conventional,
    
    /// Custom regex patterns for different types of changes.
    ///
    /// This strategy uses custom regex patterns to determine the type of change
    /// represented by a commit message.
    ///
    /// # Fields
    ///
    /// * `commit_type_pattern` - Regex pattern for extracting commit type
    /// * `title_pattern` - Regex pattern for extracting commit title
    /// * `body_pattern` - Regex pattern for extracting commit body
    /// * `breaking_pattern` - Regex pattern for commits that indicate a breaking change
    /// * `scope_pattern` - Regex pattern for extracting commit scope
    CustomRegex {
        commit_type_pattern: String,
        title_pattern: String,
        body_pattern: String,
        breaking_pattern: String,
        scope_pattern: String,
    },
}

impl Default for ParserStrategy {
    fn default() -> Self {
        ParserStrategy::Conventional
    }
}

/// Factory for creating commit parsers based on the selected strategy.
///
/// This factory creates instances of commit parsers based on the selected strategy.
/// It provides a convenient way to create parsers without having to know the details
/// of their implementation.
pub struct ParserFactory;

impl ParserFactory {
    /// Create a new commit parser based on the specified strategy.
    ///
    /// This method creates a new commit parser based on the specified strategy.
    /// If the strategy is `ParserStrategy::Conventional`, it creates a `ConventionalCommitParser`.
    /// If the strategy is `ParserStrategy::CustomRegex`, it creates a `CustomRegexParser` with
    /// the specified regex patterns.
    ///
    /// # Arguments
    ///
    /// * `strategy` - The parser strategy to use
    ///
    /// # Returns
    ///
    /// A boxed instance of a type that implements the `CommitParser` trait
    pub fn create(strategy: &ParserStrategy) -> Box<dyn CommitParser> {
        match strategy {
            ParserStrategy::Conventional => {
                log::debug!("Using conventional commit parser");
                Box::new(ConventionalCommitParser::new())
            },
            ParserStrategy::CustomRegex {
                commit_type_pattern,
                title_pattern,
                body_pattern,
                breaking_pattern,
                scope_pattern
            } => {
                log::debug!("Using custom regex parser with patterns:");
                log::debug!("  Commit type pattern: {}", commit_type_pattern);
                log::debug!("  Title pattern: {}", title_pattern);
                log::debug!("  Body pattern: {}", body_pattern);
                log::debug!("  Breaking pattern: {}", breaking_pattern);
                log::debug!("  Scope pattern: {}", scope_pattern);
                
                match CustomRegexParser::new(
                    commit_type_pattern,
                    title_pattern,
                    body_pattern,
                    breaking_pattern,
                    scope_pattern
                ) {
                    Ok(parser) => Box::new(parser),
                    Err(e) => {
                        // Fall back to default patterns if custom patterns are invalid
                        log::warn!("Invalid regex patterns, falling back to defaults: {}", e);
                        log::debug!("Using default regex patterns:");
                        log::debug!("  Commit type pattern: {}", crate::parsers::custom::COMMIT_TYPE_REGEX_STR);
                        log::debug!("  Title pattern: {}", crate::parsers::custom::TITLE_REGEX_STR);
                        log::debug!("  Body pattern: {}", crate::parsers::custom::BODY_REGEX_STR);
                        log::debug!("  Breaking pattern: {}", crate::parsers::custom::BREAKING_REGEX_STR);
                        log::debug!("  Scope pattern: {}", crate::parsers::custom::SCOPE_REGEX_STR);
                        Box::new(CustomRegexParser::default())
                    }
                }
            }
        }
    }
}