//! Utility functions and helpers.
//!
//! This module contains utility functions and helpers that are used across
//! the application but don't represent core business logic.

pub mod logging;
pub mod regex;

// Re-export commonly used functions
pub use logging::init_logging;
pub use regex::{compile_regexes, BREAKING_REGEX_STR, MAJOR_REGEX_STR, MINOR_REGEX_STR, NOOP_REGEX_STR};