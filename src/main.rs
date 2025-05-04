use clap::Parser;
use git2::{Commit, Repository};
use regex::Regex;
use semver::{BuildMetadata, Prerelease, Version};
use std::collections::HashMap;

mod logging;

// Constant regex string literals used for defaults.
const MAJOR_REGEX_STR: &str = r"(?m)^(?:major(?:\([^)]+\))?:.*|BREAKING CHANGE:.*)";
const MINOR_REGEX_STR: &str = r"(?m)^(?:minor(?:\([^)]+\))?:.*|feat(?:\([^)]+\))?:.*)$";
const NOOP_REGEX_STR: &str = r"(?m)^(?:noop(?:\([^)]+\))?:.*|chore(?:\([^)]+\))?:.*)$";
const FIX_REGEX_STR: &str = r"(?m)^fix(?:\([^)]+\))?:.*$";
const COMMIT_PATTERN: &str = r"^(major|minor|feat|fix|chore|noop)(?:\(([^)]+)\))?:\s*(.*)$|^BREAKING CHANGE:\s*(.*)$";

#[derive(Parser)]
#[clap(author, version, about = "Calculate the next version based on conventional commits")]
struct Cli {
    /// Regex for commits triggering a major version bump
    #[clap(long, default_value = MAJOR_REGEX_STR)]
    major: String,

    /// Regex for commits triggering a minor version bump
    #[clap(long, default_value = MINOR_REGEX_STR)]
    minor: String,

    /// Regex for commits that should not trigger a version bump
    #[clap(long, default_value = NOOP_REGEX_STR)]
    noop: String,

    /// Generate a markdown-formatted changelog
    #[clap(long)]
    changelog: bool,
}

struct VersionBump {
    major: bool,
    minor: bool,
    patch: bool,
}

/// Stores commit messages by category for a specific scope
struct CategoryCommits {
    features: Vec<String>,
    fixes: Vec<String>,
    chores: Vec<String>,
}

impl CategoryCommits {
    fn new() -> Self {
        CategoryCommits {
            features: Vec::new(),
            fixes: Vec::new(),
            chores: Vec::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.features.is_empty() && self.fixes.is_empty() && self.chores.is_empty()
    }
}

struct CommitSummary {
    major: u32,
    minor: u32,
    patch: u32,
    noop: u32,
    breaking_changes: Vec<String>,
    scoped_commits: HashMap<String, CategoryCommits>,
}

impl CommitSummary {
    fn new() -> Self {
        CommitSummary {
            major: 0,
            minor: 0,
            patch: 0,
            noop: 0,
            breaking_changes: Vec::new(),
            scoped_commits: HashMap::new(),
        }
    }
}

fn main() {
    logging::init_logging().expect("Failed to initialize logging");
    log::debug!("Starting vnext...");

    let cli = Cli::parse();

    log::debug!("Major bump regex: {}", cli.major);
    log::debug!("Minor bump regex: {}", cli.minor);
    log::debug!("No-op regex: {}", cli.noop);
    log::debug!("Changelog mode: {}", cli.changelog);

    let major_re = Regex::new(&cli.major).unwrap_or_else(|e| {
        log::error!("Invalid major regex '{}': {}", cli.major, e);
        std::process::exit(1);
    });
    let minor_re = Regex::new(&cli.minor).unwrap_or_else(|e| {
        log::error!("Invalid minor regex '{}': {}", cli.minor, e);
        std::process::exit(1);
    });
    let noop_re = Regex::new(&cli.noop).unwrap_or_else(|e| {
        log::error!("Invalid noop regex '{}': {}", cli.noop, e);
        std::process::exit(1);
    });
    let fix_re = Regex::new(FIX_REGEX_STR).unwrap_or_else(|e| {
        log::error!("Invalid fix regex '{}': {}", FIX_REGEX_STR, e);
        std::process::exit(1);
    });

    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => {
            log::debug!("No Git repository found: {}. Assuming version 0.0.0.", e);
            println!("0.0.0");
            std::process::exit(0);
        }
    };

    let head = match repo.head() {
        Ok(head_ref) => match head_ref.peel_to_commit() {
            Ok(commit) => commit,
            Err(e) => {
                log::debug!(
                    "Failed to resolve HEAD to a commit: {}. Assuming version 0.0.0.",
                    e
                );
                println!("0.0.0");
                std::process::exit(0);
            }
        },
        Err(e) => {
            log::debug!(
                "Failed to get HEAD: {}. Assuming version 0.0.0.",
                e
            );
            println!("0.0.0");
            std::process::exit(0);
        }
    };
    log::debug!("HEAD commit: {}", head.id());

    let main_branch = find_main_branch(&repo).expect("Failed to find main branch");
    log::debug!("Main branch detected: {}", main_branch);

    let (start_version, last_tag_commit) = match find_latest_tag(&repo) {
        Some((tag, commit)) => {
            let version = parse_version(&tag).unwrap_or_else(|_| Version::new(0, 0, 0));
            log::debug!("Last release: {} at commit {}", tag, commit.id());
            (version, commit)
        }
        None => {
            log::debug!("No previous release tags found, starting from 0.0.0");
            let version = Version::new(0, 0, 0);
            let parents = head.parents();
            let base_commit = if parents.count() > 0 {
                let mut earliest = head.clone();
                for parent in head.parents() {
                    earliest = parent.clone(); // Traverse to the root
                }
                earliest
            } else {
                head.clone()
            };
            (version, base_commit)
        }
    };
    log::debug!("Last tag or base commit: {}", last_tag_commit.id());

    // Determine the base commit: use merge base with main if tag exists, otherwise earliest commit
    let base_commit = if find_latest_tag(&repo).is_some() {
        let merge_base = repo
            .merge_base(head.id(), last_tag_commit.id())
            .expect("Failed to find merge base between HEAD and tag");
        repo.find_commit(merge_base)
            .expect("Failed to find merge base commit")
    } else {
        last_tag_commit.clone()
    };
    log::debug!("Base commit for analysis: {}", base_commit.id());

    let (bump, summary) = calculate_version_bump(&repo, &base_commit, &head, &major_re, &minor_re, &noop_re, &fix_re);

    log::debug!(
        "Commits pending release: {} major, {} minor, {} patch, {} noop",
        summary.major, summary.minor, summary.patch, summary.noop
    );

    let next_version = calculate_next_version(&start_version, &bump);
    log::debug!(
        "Version bump: major={}, minor={}, patch={}",
        bump.major, bump.minor, bump.patch
    );
    log::debug!("Next version: {}", next_version);

    if cli.changelog {
        let changelog = generate_changelog(&next_version, &summary);
        println!("{}", changelog);
    } else {
        println!("{}", next_version);
    }
}

fn find_main_branch(repo: &Repository) -> Option<String> {
    for branch in ["main", "master"] {
        if repo.find_branch(branch, git2::BranchType::Local).is_ok() {
            return Some(branch.to_string());
        }
    }
    None
}

fn find_latest_tag(repo: &Repository) -> Option<(String, Commit)> {
    let tags = repo.tag_names(None).expect("Failed to get tag names");
    let mut latest_version: Option<(String, Commit)> = None;
    let mut max_version = Version::new(0, 0, 0);

    for tag in tags.iter().flatten() {
        if let Ok(tag_ref) = repo.find_reference(&format!("refs/tags/{}", tag)) {
            if let Ok(commit) = tag_ref.peel_to_commit() {
                if let Ok(version) = parse_version(tag) {
                    if version > max_version {
                        max_version = version;
                        latest_version = Some((tag.to_string(), commit));
                    }
                }
            }
        }
    }
    latest_version
}

fn parse_version(tag: &str) -> Result<Version, semver::Error> {
    let cleaned_tag = tag.trim_start_matches('v');
    Version::parse(cleaned_tag)
}

fn calculate_version_bump(
    _repo: &Repository,
    from: &Commit,
    to: &Commit,
    major_re: &Regex,
    minor_re: &Regex,
    noop_re: &Regex,
    fix_re: &Regex,
) -> (VersionBump, CommitSummary) {
    let mut bump = VersionBump {
        major: false,
        minor: false,
        patch: false,
    };
    let mut summary = CommitSummary::new();
    
    // Compile the regex for parsing commit messages
    let commit_pattern = Regex::new(COMMIT_PATTERN).unwrap();
    let mut commit_count = 0;

    let mut current = to.clone();
    let base_id = from.id();
    let mut seen = std::collections::HashSet::new();

    log::debug!("Walking commits from {} to base {}", to.id(), base_id);

    // Special case: if base_id and to.id() are the same (single commit repo),
    // analyze the commit itself
    if to.id() == base_id {
        log::debug!("Single commit repo, analyzing the commit itself");
        let message = to.message().unwrap_or("");
        log::debug!(
            "Analyzing commit: {} - {}",
            to.id(),
            message.lines().next().unwrap_or("")
        );
        
        process_commit_message(&mut bump, &mut summary, message, major_re, minor_re, noop_re, fix_re, &commit_pattern);
        
        commit_count = 1;
    } else {
        // Walk from HEAD until we reach the base commit, excluding the base itself.
        while current.id() != base_id {
            if seen.contains(&current.id()) {
                break; // Avoid infinite loops.
            }
            seen.insert(current.id());
            commit_count += 1;

            let message = current.message().unwrap_or("");
            log::debug!(
                "Pending commit: {} - {}",
                current.id(),
                message.lines().next().unwrap_or("")
            );

            process_commit_message(&mut bump, &mut summary, message, major_re, minor_re, noop_re, fix_re, &commit_pattern);

            if current.parents().count() == 0 {
                break; // Reached the root.
            }
            current = current.parents().next().unwrap().clone();
        }
    }

    log::debug!("Total commits analyzed: {}", commit_count);
    (bump, summary)
}

fn calculate_next_version(current: &Version, bump: &VersionBump) -> Version {
    let mut next = current.clone();
    next.pre = Prerelease::EMPTY;
    next.build = BuildMetadata::EMPTY;

    if bump.major {
        next.major += 1;
        next.minor = 0;
        next.patch = 0;
    } else if bump.minor {
        next.minor += 1;
        next.patch = 0;
    } else if bump.patch {
        next.patch += 1;
    }

    next
}

fn process_commit_message(
    bump: &mut VersionBump,
    summary: &mut CommitSummary,
    message: &str,
    major_re: &Regex,
    minor_re: &Regex,
    noop_re: &Regex,
    fix_re: &Regex,
    commit_pattern: &Regex,
) {
    // Log the message being processed
    log::debug!("Processing commit message: {}", message);
    
    // Check for version bump type
    if major_re.is_match(message) {
        bump.major = true;
        summary.major += 1;
        log::debug!("Detected major bump");
        
        // If it's a major commit, also add it to breaking changes
        let first_line = message.lines().next().unwrap_or("");
        if first_line.starts_with("major:") {
            let content = first_line.trim_start_matches("major:").trim().to_string();
            log::debug!("Adding major commit to breaking changes: {}", content);
            summary.breaking_changes.push(content);
        }
    } else if minor_re.is_match(message) {
        bump.minor = true;
        summary.minor += 1;
        log::debug!("Detected minor bump");
    } else if fix_re.is_match(message) {
        bump.patch = true;
        summary.patch += 1;
        log::debug!("Detected patch bump (fix)");
    } else if !noop_re.is_match(message) {
        bump.patch = true;
        summary.patch += 1;
        log::debug!("Detected patch bump (other)");
    } else {
        summary.noop += 1;
        log::debug!("Detected no-op commit");
    }

    // Process for changelog
    // First, check for BREAKING CHANGE in the message body
    if message.contains("BREAKING CHANGE:") {
        for line in message.lines() {
            if line.starts_with("BREAKING CHANGE:") {
                let breaking_content = line.trim_start_matches("BREAKING CHANGE:").trim();
                log::debug!("Adding BREAKING CHANGE to breaking changes: {}", breaking_content);
                summary.breaking_changes.push(breaking_content.to_string());
            }
        }
    }

    // Process the first line of the commit message
    let first_line = message.lines().next().unwrap_or("");
    log::debug!("Processing first line: {}", first_line);
    
    if let Some(captures) = commit_pattern.captures(first_line) {
        // Check if it's a BREAKING CHANGE line
        if let Some(breaking_content) = captures.get(4) {
            let content = breaking_content.as_str().trim().to_string();
            log::debug!("Adding breaking content from regex capture: {}", content);
            summary.breaking_changes.push(content);
            return;
        }

        // Otherwise, it's a conventional commit
        if let (Some(commit_type), scope, Some(content)) = (
            captures.get(1),
            captures.get(2).map(|m| m.as_str()),
            captures.get(3),
        ) {
            let commit_type = commit_type.as_str();
            let scope_key = scope.unwrap_or("").to_string();
            let content = content.as_str().trim().to_string();
            
            log::debug!("Parsed commit: type={}, scope={}, content={}",
                       commit_type, scope.unwrap_or(""), content);

            // Get or create the CategoryCommits for this scope
            let category = summary.scoped_commits
                .entry(scope_key)
                .or_insert_with(CategoryCommits::new);

            // Add the commit message to the appropriate category
            match commit_type {
                "major" => {
                    log::debug!("Adding major commit to breaking changes: {}", content);
                    summary.breaking_changes.push(content);
                }
                "feat" | "minor" => {
                    log::debug!("Adding feature: {}", content);
                    category.features.push(content);
                }
                "fix" => {
                    log::debug!("Adding fix: {}", content);
                    category.fixes.push(content);
                }
                "chore" | "noop" => {
                    log::debug!("Adding chore: {}", content);
                    category.chores.push(content);
                }
                _ => {
                    // Unknown commit type, treat as patch
                    log::debug!("Adding unknown commit type as fix: {}", content);
                    category.fixes.push(content);
                }
            }
        }
    } else if first_line.starts_with("major:") {
        // Handle major: commits that don't match the pattern exactly
        let content = first_line.trim_start_matches("major:").trim().to_string();
        log::debug!("Adding major commit (fallback) to breaking changes: {}", content);
        summary.breaking_changes.push(content);
    }
}

fn generate_changelog(version: &Version, summary: &CommitSummary) -> String {
    let mut changelog = String::new();

    // Add the title with the version
    changelog.push_str(&format!("# {}\n\n", version));

    // Add breaking changes section if there are any
    if !summary.breaking_changes.is_empty() {
        changelog.push_str("## Breaking Changes\n\n");
        for change in &summary.breaking_changes {
            changelog.push_str(&format!("- {}\n", change));
        }
        changelog.push('\n');
    }

    // Process unscoped commits first
    if let Some(unscoped) = summary.scoped_commits.get("") {
        if !unscoped.is_empty() {
            changelog.push_str("## Changes\n\n");
            
            if !unscoped.features.is_empty() {
                changelog.push_str("### Features\n\n");
                for feature in &unscoped.features {
                    changelog.push_str(&format!("- {}\n", feature));
                }
                changelog.push('\n');
            }
            
            if !unscoped.fixes.is_empty() {
                changelog.push_str("### Fixes\n\n");
                for fix in &unscoped.fixes {
                    changelog.push_str(&format!("- {}\n", fix));
                }
                changelog.push('\n');
            }
            
            if !unscoped.chores.is_empty() {
                changelog.push_str("### Chores\n\n");
                for chore in &unscoped.chores {
                    changelog.push_str(&format!("- {}\n", chore));
                }
                changelog.push('\n');
            }
        }
    }

    // Process scoped commits
    let mut scopes: Vec<&String> = summary.scoped_commits.keys()
        .filter(|k| !k.is_empty())
        .collect();
    scopes.sort(); // Sort scopes alphabetically

    for scope in scopes {
        if let Some(category) = summary.scoped_commits.get(scope) {
            if !category.is_empty() {
                // Capitalize the scope for the heading
                let capitalized_scope = scope.chars().next().map(|c| c.to_uppercase().collect::<String>())
                    .unwrap_or_default() + &scope[1..];
                
                changelog.push_str(&format!("## {} Changes\n\n", capitalized_scope));
                
                if !category.features.is_empty() {
                    changelog.push_str("### Features\n\n");
                    for feature in &category.features {
                        changelog.push_str(&format!("- {}\n", feature));
                    }
                    changelog.push('\n');
                }
                
                if !category.fixes.is_empty() {
                    changelog.push_str("### Fixes\n\n");
                    for fix in &category.fixes {
                        changelog.push_str(&format!("- {}\n", fix));
                    }
                    changelog.push('\n');
                }
                
                if !category.chores.is_empty() {
                    changelog.push_str("### Chores\n\n");
                    for chore in &category.chores {
                        changelog.push_str(&format!("- {}\n", chore));
                    }
                    changelog.push('\n');
                }
            }
        }
    }

    changelog
}

#[cfg(test)]
mod tests {
    use super::*;
    use git2::Repository;
    use semver::Version;
    use tempfile;

    #[test]
    fn test_parse_version() {
        assert_eq!(parse_version("v1.2.3").unwrap(), Version::new(1, 2, 3));
        assert_eq!(parse_version("1.2.3").unwrap(), Version::new(1, 2, 3));
        assert!(parse_version("invalid").is_err());
    }

    #[test]
    fn test_calculate_next_version() {
        let base = Version::new(1, 2, 3);

        let bump = VersionBump {
            major: false,
            minor: false,
            patch: false,
        };
        assert_eq!(calculate_next_version(&base, &bump), Version::new(1, 2, 3));

        let bump = VersionBump {
            major: false,
            minor: false,
            patch: true,
        };
        assert_eq!(calculate_next_version(&base, &bump), Version::new(1, 2, 4));

        let bump = VersionBump {
            major: false,
            minor: true,
            patch: false,
        };
        assert_eq!(calculate_next_version(&base, &bump), Version::new(1, 3, 0));

        let bump = VersionBump {
            major: true,
            minor: false,
            patch: false,
        };
        assert_eq!(calculate_next_version(&base, &bump), Version::new(2, 0, 0));
    }

    #[test]
    fn test_regex_patterns() {
        // Compile regexes from the constant strings in each test.
        let major_re = Regex::new(MAJOR_REGEX_STR).unwrap();
        let minor_re = Regex::new(MINOR_REGEX_STR).unwrap();
        let noop_re = Regex::new(NOOP_REGEX_STR).unwrap();

        // Major regex tests
        assert!(major_re.is_match("major: update something"));
        assert!(major_re.is_match("major(scope): big change"));
        assert!(major_re.is_match("BREAKING CHANGE: this is major"));
        assert!(!major_re.is_match("feat: non-breaking"));
        assert!(!major_re.is_match("minor: something"));
        assert!(!major_re.is_match("chore: cleanup"));

        // Minor regex tests
        assert!(minor_re.is_match("minor: add feature"));
        assert!(minor_re.is_match("minor(scope): add feature"));
        assert!(minor_re.is_match("feat: add feature"));
        assert!(minor_re.is_match("feat(scope): add feature"));
        assert!(!minor_re.is_match("major: update"));
        assert!(!minor_re.is_match("chore: cleanup"));

        // No-op regex tests
        assert!(noop_re.is_match("noop: nothing big"));
        assert!(noop_re.is_match("noop(scope): nothing big"));
        assert!(noop_re.is_match("chore: cleanup"));
        assert!(noop_re.is_match("chore(scope): cleanup"));
        assert!(!noop_re.is_match("feat: add feature"));
        assert!(!noop_re.is_match("major: update"));
    }

    #[test]
    fn test_generate_changelog() {
        // Create a test version
        let version = Version::new(1, 0, 0);
        
        // Create a test commit summary with various types of changes
        let mut summary = CommitSummary::new();
        
        // Add breaking changes
        summary.breaking_changes.push("Big refactor".to_string());
        summary.breaking_changes.push("API removed".to_string());
        
        // Add unscoped commits
        let unscoped = summary.scoped_commits.entry("".to_string()).or_insert_with(CategoryCommits::new);
        unscoped.features.push("Add new widget".to_string());
        unscoped.fixes.push("Fix bug in widget".to_string());
        unscoped.chores.push("Update docs".to_string());
        
        // Add UI scoped commits
        let ui_scoped = summary.scoped_commits.entry("ui".to_string()).or_insert_with(CategoryCommits::new);
        ui_scoped.features.push("Add new button".to_string());
        ui_scoped.fixes.push("Fix button alignment".to_string());
        
        // Add API scoped commits with only features
        let api_scoped = summary.scoped_commits.entry("api".to_string()).or_insert_with(CategoryCommits::new);
        api_scoped.features.push("Add new endpoint".to_string());
        
        // Generate the changelog
        let changelog = generate_changelog(&version, &summary);
        
        // Verify the changelog content
        assert!(changelog.starts_with("# 1.0.0\n\n"), "Changelog should start with the version");
        
        // Check for breaking changes section
        assert!(changelog.contains("## Breaking Changes\n\n"), "Changelog should have a Breaking Changes section");
        assert!(changelog.contains("- Big refactor\n"), "Breaking changes should include 'Big refactor'");
        assert!(changelog.contains("- API removed\n"), "Breaking changes should include 'API removed'");
        
        // Check for unscoped changes section
        assert!(changelog.contains("## Changes\n\n"), "Changelog should have a Changes section");
        assert!(changelog.contains("### Features\n\n- Add new widget\n"), "Changes should include feature 'Add new widget'");
        assert!(changelog.contains("### Fixes\n\n- Fix bug in widget\n"), "Changes should include fix 'Fix bug in widget'");
        assert!(changelog.contains("### Chores\n\n- Update docs\n"), "Changes should include chore 'Update docs'");
        
        // Check for UI scoped changes section
        assert!(changelog.contains("## Ui Changes\n\n"), "Changelog should have a UI Changes section");
        assert!(changelog.contains("### Features\n\n- Add new button\n"), "UI Changes should include feature 'Add new button'");
        assert!(changelog.contains("### Fixes\n\n- Fix button alignment\n"), "UI Changes should include fix 'Fix button alignment'");
        
        // Check for API scoped changes section
        assert!(changelog.contains("## Api Changes\n\n"), "Changelog should have an API Changes section");
        assert!(changelog.contains("### Features\n\n- Add new endpoint\n"), "API Changes should include feature 'Add new endpoint'");
        
        // Verify the order of sections (Breaking Changes first, then unscoped, then scoped alphabetically)
        let breaking_pos = changelog.find("## Breaking Changes").unwrap_or(0);
        let changes_pos = changelog.find("## Changes").unwrap_or(0);
        let api_pos = changelog.find("## Api Changes").unwrap_or(0);
        let ui_pos = changelog.find("## Ui Changes").unwrap_or(0);
        
        assert!(breaking_pos < changes_pos, "Breaking Changes should come before Changes");
        assert!(changes_pos < api_pos, "Changes should come before Api Changes");
        assert!(api_pos < ui_pos, "Api Changes should come before Ui Changes (alphabetical order)");
    }

    #[test]
    fn test_calculate_version_bump() {
        let temp_dir = tempfile::tempdir().unwrap();
        let repo_path = temp_dir.path().join("test_repo");
        let repo = Repository::init_bare(&repo_path).unwrap();
        let sig = git2::Signature::now("Test", "test@example.com").unwrap();
        let mut index = repo.index().unwrap();
        let tree_id = index.write_tree().unwrap();
        let tree = repo.find_tree(tree_id).unwrap();

        let base_commit_id = repo
            .commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])
            .unwrap();
        let base_commit = repo.find_commit(base_commit_id).unwrap();

        let test_cases = vec![
            // Original test cases
            ("major: big update", true, false, false),          // Major
            ("major(scope): major change", true, false, false),   // Major with scope
            ("BREAKING CHANGE: major change", true, false, false), // Major
            ("minor: new feature", false, true, false),           // Minor
            ("feat: add stuff", false, true, false),              // Minor
            ("feat(scope): add stuff", false, true, false),       // Minor with scope
            ("noop: nothing", false, false, false),               // No-op
            ("chore(scope): cleanup", false, false, false),       // No-op with scope
            ("fix: bugfix", false, false, true),                  // Patch
            // Semantic Release examples
            (
                "fix(pencil): stop graphite breaking when too much pressure applied",
                false,
                false,
                true,
            ), // Patch
            (
                "feat(pencil): add 'graphiteWidth' option",
                false,
                true,
                false,
            ), // Minor
            (
                "perf(pencil): remove graphiteWidth option\n\nBREAKING CHANGE: The graphiteWidth option has been removed.\nThe default graphite width of 10mm is always used for performance reasons.",
                true,
                false,
                false,
            ), // Major
        ];

        let major_re = Regex::new(MAJOR_REGEX_STR).unwrap();
        let minor_re = Regex::new(MINOR_REGEX_STR).unwrap();
        let noop_re = Regex::new(NOOP_REGEX_STR).unwrap();
        let fix_re = Regex::new(FIX_REGEX_STR).unwrap();

        for (message, expect_major, expect_minor, expect_patch) in test_cases {
            let to_commit_id = repo
                .commit(Some("HEAD"), &sig, &sig, message, &tree, &[&base_commit])
                .unwrap();
            let to_commit = repo.find_commit(to_commit_id).unwrap();

            let (bump, summary) =
                calculate_version_bump(&repo, &base_commit, &to_commit, &major_re, &minor_re, &noop_re, &fix_re);

            assert_eq!(bump.major, expect_major, "Message: {}", message);
            assert_eq!(bump.minor, expect_minor, "Message: {}", message);
            assert_eq!(bump.patch, expect_patch, "Message: {}", message);

            assert_eq!(
                summary.major,
                if expect_major { 1 } else { 0 },
                "Message: {}",
                message
            );
            assert_eq!(
                summary.minor,
                if expect_minor { 1 } else { 0 },
                "Message: {}",
                message
            );
            assert_eq!(
                summary.patch,
                if expect_patch { 1 } else { 0 },
                "Message: {}",
                message
            );
            assert_eq!(
                summary.noop,
                if !expect_major && !expect_minor && !expect_patch {
                    1
                } else {
                    0
                },
                "Message: {}",
                message
            );

            // Reset HEAD for the next test.
            repo.reference("HEAD", base_commit.id(), true, "Reset for next test")
                .unwrap();
        }
    }
}
