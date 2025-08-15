# Architecture

This document provides a high-level overview of the vnext application architecture.

## Overview

vnext is a CLI tool for semantic versioning and changelog generation. It analyzes Git commit history to determine the next version number based on conventional commit messages and generates a changelog.

## Components

### CLI Interface

The CLI interface is defined in `src/cli.rs` using the clap crate. It defines the command-line arguments and subcommands.

### Models

The `src/models` directory contains all data structures used throughout the application. These are organized by domain:

- `error.rs`: Error types like `VNextError`
- `version.rs`: Version-related structures like `VersionBump`
- `commit.rs`: Commit-related structures like `CommitSummary` and `CommitAuthor`
- `repo.rs`: Repository information structures like `RepoInfo`
- `github.rs`: GitHub-related structures
- `deploy_key.rs`: Deploy key related structures

### Utils

The `src/utils` directory contains utility functions and helpers that are used across the application but don't represent core business logic:

- `logging.rs`: Logging setup and configuration
- `regex.rs`: Regex pattern compilation and constants
- `git.rs`: Git utility functions

### Services

The `src/services` directory contains the core business logic of the application, organized by domain:

- `git.rs`: Git repository operations
- `github.rs`: GitHub API integration
- `version.rs`: Version calculation logic
- `changelog.rs`: Changelog generation

### Commands

The `src/commands` directory contains the implementation of each CLI command defined in the `Commands` enum in cli.rs:

- `deploy_key.rs`: Deploy key command implementation
- `version.rs`: Version calculation command implementation

## Data Flow

1. The user invokes the CLI with arguments
2. The CLI parses the arguments and determines which command to run
3. The command implementation uses services to perform the requested operation
4. Services use models to represent data and utils for common functionality
5. The result is returned to the user