# Simplified Plan: Making Commit Types Configurable

## Problem Statement
Currently, the commit types that trigger version bumps (major, minor, noop) are hardcoded in the `Commit` struct methods. We want to make these configurable via command-line arguments.

## Simplified Design Approach
We'll simplify the `Commit` struct to focus on data only, with a single `has_breaking_change` property. The logic for determining version bumps will be moved to a separate function that uses the configurable commit types.

## Implementation Plan

### 1. Simplify the `Commit` Struct
Modify the `Commit` struct in `src/models/commit.rs`:

```rust
#[derive(Clone, Debug)]
pub struct Commit {
    pub commit_id: String,
    pub raw_message: String,
    pub commit_type: String,
    pub scope: Option<String>,
    pub has_breaking_change: bool,  // Single flag for breaking changes
    pub title: String,
    pub body: Option<String>,
    pub author: Option<CommitAuthor>,
}

impl Commit {
    /// Create a new Commit instance with minimal information
    pub fn new(commit_id: String, raw_message: String) -> Self {
        Commit {
            commit_id,
            raw_message,
            commit_type: String::new(),
            scope: None,
            has_breaking_change: false,
            title: String::new(),
            body: None,
            author: None,
        }
    }
    
    /// Parse a commit message using the conventional commit format
    pub fn parse(commit_id: String, message: String) -> Self {
        let mut commit = Commit::new(commit_id, message.clone());
        
        if let Some(parsed) = crate::parsers::conventional::parse_conventional_commit(&message) {
            commit.commit_type = parsed.commit_type;
            commit.scope = parsed.scope;
            // Set has_breaking_change if either flag or body indicates a breaking change
            commit.has_breaking_change = parsed.breaking_change_flag || parsed.breaking_change_body;
            commit.title = parsed.title;
            commit.body = parsed.body;
        }
        
        commit
    }
}
```

### 2. Create a `VersionBumpAnalyzer` Function
Create a new function in `src/core/version.rs` that determines the version bump based on commit type and configurable settings:

```rust
/// Determine the type of version bump for a commit based on configurable commit types
pub fn determine_version_bump(
    commit: &Commit,
    major_types: &[&str],
    minor_types: &[&str],
    noop_types: &[&str]
) -> VersionBumpType {
    // Breaking changes always trigger a major bump
    if commit.has_breaking_change {
        return VersionBumpType::Major;
    }
    
    // Check commit type against configurable lists
    if major_types.contains(&commit.commit_type.as_str()) {
        VersionBumpType::Major
    } else if minor_types.contains(&commit.commit_type.as_str()) {
        VersionBumpType::Minor
    } else if noop_types.contains(&commit.commit_type.as_str()) {
        VersionBumpType::NoOp
    } else {
        // Default to patch for any other commit type
        VersionBumpType::Patch
    }
}

/// Enum representing the type of version bump
pub enum VersionBumpType {
    Major,
    Minor,
    Patch,
    NoOp,
}
```

### 3. Update the `calculate_version_bump` Function
Modify the `calculate_version_bump` function in `src/core/version.rs` to use the new `determine_version_bump` function:

```rust
pub fn calculate_version_bump(
    repo: &Repository,
    _from: &Commit,
    to: &Commit,
    parser: &dyn crate::models::commit::CommitParser,
    major_types: &[&str],
    minor_types: &[&str],
    noop_types: &[&str]
) -> Result<(VersionBump, ChangesetSummary), VNextError> {
    // ...
    
    // For each commit, determine the version bump
    for oid in revwalk {
        let oid = oid?;
        let git_commit = repo.find_commit(oid)?;
        let message = git_commit.message().unwrap_or("").to_string();
        
        // Parse the commit message
        let commit = parser.parse_commit(oid.to_string(), message);
        
        // Determine the version bump
        match determine_version_bump(&commit, major_types, minor_types, noop_types) {
            VersionBumpType::Major => {
                bump.major = true;
                summary.major += 1;
                log::debug!("Detected major change in commit: {}", commit.commit_id);
            },
            VersionBumpType::Minor => {
                bump.minor = true;
                summary.minor += 1;
                log::debug!("Detected minor change in commit: {}", commit.commit_id);
            },
            VersionBumpType::Patch => {
                bump.patch = true;
                summary.patch += 1;
                log::debug!("Detected patch change in commit: {}", commit.commit_id);
            },
            VersionBumpType::NoOp => {
                summary.noop += 1;
                log::debug!("Detected no-op change in commit: {}", commit.commit_id);
            }
        }
        
        // Add the commit to the summary
        summary.commits.push(commit);
    }
    
    // ...
}
```

### 4. Add CLI Arguments
Add command-line arguments to the `Cli` struct in `src/cli.rs`:

```rust
/// Comma-separated list of commit types that trigger a major version bump
#[clap(long, default_value = "major")]
pub major_commit_types: String,

/// Comma-separated list of commit types that trigger a minor version bump
#[clap(long, default_value = "feat,minor")]
pub minor_commit_types: String,

/// Comma-separated list of commit types that should not trigger a version bump
#[clap(long, default_value = "chore,noop")]
pub noop_commit_types: String,
```

### 5. Update the `run_vnext_command` Function
Modify the `run_vnext_command` function in `src/commands/vnext.rs` to parse the commit types and pass them to the `calculate_version_bump` function:

```rust
pub fn run_vnext_command(
    parser_name: &str,
    major_pattern: &str,
    minor_pattern: &str,
    noop_pattern: &str,
    breaking_pattern: &str,
    type_pattern: &str,
    scope_pattern: &str,
    major_commit_types: &str,
    minor_commit_types: &str,
    noop_commit_types: &str,
    show_changelog: bool,
    no_header_scaling: bool,
    current: bool,
) -> Result<(), VNextError> {
    // Parse comma-separated commit types
    let major_types: Vec<&str> = major_commit_types.split(',').map(|s| s.trim()).collect();
    let minor_types: Vec<&str> = minor_commit_types.split(',').map(|s| s.trim()).collect();
    let noop_types: Vec<&str> = noop_commit_types.split(',').map(|s| s.trim()).collect();
    
    // ...
    
    // Calculate version
    let (next_version, mut summary) = match version::calculate_version(
        &repo, &head, &current_version, &base_commit, &*parser,
        &major_types, &minor_types, &noop_types
    ) {
        // ...
    };
    
    // ...
}
```

### 6. Update the `calculate_version` Function
Modify the `calculate_version` function in `src/core/version.rs` to pass the commit types to `calculate_version_bump`:

```rust
pub fn calculate_version(
    repo: &Repository,
    head: &Commit,
    start_version: &Version,
    base_commit: &Commit,
    parser: &dyn crate::models::commit::CommitParser,
    major_types: &[&str],
    minor_types: &[&str],
    noop_types: &[&str]
) -> Result<(Version, ChangesetSummary), VNextError> {
    // Calculate version bump
    let (bump, summary) = calculate_version_bump(
        repo, base_commit, head, parser, major_types, minor_types, noop_types
    )?;
    
    // ...
}
```

### 7. Update the `ConventionalCommitParser`
Modify the `ConventionalCommitParser` in `src/parsers/conventional.rs` to set the `has_breaking_change` property:

```rust
impl CommitParser for ConventionalCommitParser {
    fn parse_commit(&self, commit_id: String, message: String) -> Commit {
        let mut commit = Commit::new(commit_id, message.clone());
        
        if let Some(parsed) = parse_conventional_commit(&message) {
            commit.commit_type = parsed.commit_type;
            commit.scope = parsed.scope;
            // Set has_breaking_change if either flag or body indicates a breaking change
            commit.has_breaking_change = parsed.breaking_change_flag || parsed.breaking_change_body;
            commit.title = parsed.title;
            commit.body = parsed.body;
        }
        
        commit
    }
    
    // ...
}
```

### 8. Add Tests
Add tests for the new functionality:
- Test the `determine_version_bump` function with different commit types
- Test the behavior with different configurations

## Benefits of This Approach
1. **Simplified Commit Struct**: The `Commit` struct focuses on data only
2. **Clear Separation of Concerns**: Logic for determining version bumps is separate from the data
3. **Configurable**: Commit types are configurable via command-line arguments
4. **Testable**: Easy to test with different configurations
5. **No Global State**: Avoids static variables and global state

## Migration Strategy
1. Simplify the `Commit` struct to have a single `has_breaking_change` property
2. Implement the new version bump determination logic
3. Update the main code path to use the new functionality
4. Update tests to use the new approach