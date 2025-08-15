//! Utility functions and helpers.
//!
//! This module contains utility functions and helpers that are used across
//! the application but don't represent core business logic.

pub mod logging;

// Re-export commonly used functions
pub use logging::init_logging;