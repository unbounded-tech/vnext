//! CLI command implementations.
//!
//! This module contains the implementation of each CLI command defined in
//! the `Commands` enum in cli.rs.

pub mod deploy_key;
pub mod version;

// Re-export command functions
pub use deploy_key::generate_deploy_key;
pub use version::run_version_command;