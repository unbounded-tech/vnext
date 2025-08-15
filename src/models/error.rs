//! Error types for the vnext application

use std::fmt;

/// Custom error type for vnext
#[derive(Debug)]
pub enum VNextError {
    /// Git-related errors
    GitError(git2::Error),
    /// IO-related errors
    IoError(std::io::Error),
    /// Regex-related errors
    RegexError(regex::Error),
    /// GitHub API-related errors
    GithubError(String),
    /// Version parsing errors
    VersionError(semver::Error),
    /// Other errors
    Other(String),
}

impl fmt::Display for VNextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VNextError::GitError(e) => write!(f, "Git error: {}", e),
            VNextError::IoError(e) => write!(f, "IO error: {}", e),
            VNextError::RegexError(e) => write!(f, "Regex error: {}", e),
            VNextError::GithubError(e) => write!(f, "GitHub API error: {}", e),
            VNextError::VersionError(e) => write!(f, "Version parsing error: {}", e),
            VNextError::Other(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for VNextError {}

// Implement conversions from other error types
impl From<git2::Error> for VNextError {
    fn from(err: git2::Error) -> Self {
        VNextError::GitError(err)
    }
}

impl From<std::io::Error> for VNextError {
    fn from(err: std::io::Error) -> Self {
        VNextError::IoError(err)
    }
}

impl From<regex::Error> for VNextError {
    fn from(err: regex::Error) -> Self {
        VNextError::RegexError(err)
    }
}

impl From<semver::Error> for VNextError {
    fn from(err: semver::Error) -> Self {
        VNextError::VersionError(err)
    }
}

impl From<reqwest::Error> for VNextError {
    fn from(err: reqwest::Error) -> Self {
        VNextError::GithubError(err.to_string())
    }
}

impl From<String> for VNextError {
    fn from(err: String) -> Self {
        VNextError::Other(err)
    }
}

impl From<&str> for VNextError {
    fn from(err: &str) -> Self {
        VNextError::Other(err.to_string())
    }
}