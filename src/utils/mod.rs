//! Utility functions and helpers.
//!
//! This module contains utility functions and helpers that are used across
//! the application but don't represent core business logic.

pub mod logging;
pub mod regex;

// Re-export commonly used functions
pub use logging::init_logging;
pub use regex::parse_conventional_commit;