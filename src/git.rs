use git2::{Commit, Repository};
use regex::Regex;
use crate::version::{CommitSummary, VersionBump};

pub fn find_main_branch(repo: &Repository) -> Option<String> {
    for branch in ["main", "master"] {
        if repo.find_branch(branch, git2::BranchType::Local).is_ok() {
            return Some(branch.to_string());
        }
    }
    None
}

pub fn find_latest_tag(repo: &Repository) -> Option<(String, Commit)> {
    let tags = repo.tag_names(None).expect("Failed to get tag names");
    let mut latest_version: Option<(String, Commit)> = None;
    let mut max_version = crate::version::parse_version("0.0.0").unwrap();

    for tag in tags.iter().flatten() {
        if let Ok(tag_ref) = repo.find_reference(&format!("refs/tags/{}", tag)) {
            if let Ok(commit) = tag_ref.peel_to_commit() {
                if let Ok(version) = crate::version::parse_version(tag) {
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

pub fn calculate_version_bump(
    _repo: &Repository,
    from: &Commit,
    to: &Commit,
    major_re: &Regex,
    minor_re: &Regex,
    noop_re: &Regex,
    breaking_re: &Regex,
) -> (VersionBump, CommitSummary) {
    let mut bump = VersionBump {
        major: false,
        minor: false,
        patch: false,
    };
    let mut summary = CommitSummary::new();
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
        let message_str = message.to_string();
        let first_line = message.lines().next().unwrap_or("").to_string();
        log::debug!(
            "Analyzing commit: {} - {}",
            to.id(),
            first_line
        );
        
        if breaking_re.is_match(message) || major_re.is_match(message) {
            bump.major = true;
            summary.major += 1;
            summary.commits.push((to.id().to_string(), message_str));
        } else if minor_re.is_match(message) {
            bump.minor = true;
            summary.minor += 1;
            summary.commits.push((to.id().to_string(), message_str));
        } else if !noop_re.is_match(message) {
            bump.patch = true;
            summary.patch += 1;
            summary.commits.push((to.id().to_string(), message_str));
        } else {
            summary.noop += 1;
            summary.commits.push((to.id().to_string(), message_str));
        }
        
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
            let message_str = message.to_string();
            let first_line = message.lines().next().unwrap_or("").to_string();
            log::debug!(
                "Pending commit: {} - {}",
                current.id(),
                first_line
            );

            if breaking_re.is_match(message) || major_re.is_match(message) {
                bump.major = true;
                summary.major += 1;
                summary.commits.push((current.id().to_string(), message_str));
            } else if minor_re.is_match(message) {
                bump.minor = true;
                summary.minor += 1;
                summary.commits.push((current.id().to_string(), message_str));
            } else if !noop_re.is_match(message) {
                bump.patch = true;
                summary.patch += 1;
                summary.commits.push((current.id().to_string(), message_str));
            } else {
                summary.noop += 1;
                summary.commits.push((current.id().to_string(), message_str));
            }

            if current.parents().count() == 0 {
                break; // Reached the root.
            }
            current = current.parents().next().unwrap().clone();
        }
    }

    log::debug!("Total commits analyzed: {}", commit_count);
    (bump, summary)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::{MAJOR_REGEX_STR, MINOR_REGEX_STR, NOOP_REGEX_STR, BREAKING_REGEX_STR};
    use regex::Regex;
    use tempfile;

    #[test]
    fn test_regex_patterns() {
        // Compile regexes from the constant strings in each test.
        let major_re = Regex::new(MAJOR_REGEX_STR).unwrap();
        let minor_re = Regex::new(MINOR_REGEX_STR).unwrap();
        let noop_re = Regex::new(NOOP_REGEX_STR).unwrap();
        let breaking_re = Regex::new(BREAKING_REGEX_STR).unwrap();

        // Major regex tests
        assert!(major_re.is_match("major: update something"));
        assert!(major_re.is_match("major(scope): big change"));
        assert!(!major_re.is_match("BREAKING CHANGE: this is major")); // Now handled by breaking_re
        assert!(!major_re.is_match("feat: non-breaking"));
        assert!(!major_re.is_match("minor: something"));
        assert!(!major_re.is_match("chore: cleanup"));

        // Breaking change regex tests
        assert!(breaking_re.is_match("BREAKING CHANGE: this is major"));
        assert!(breaking_re.is_match("feat: add stuff\nBREAKING CHANGE: old stuff removed"));
        assert!(breaking_re.is_match("fix: bugfix\nBREAKING CHANGE: changed behavior"));
        assert!(!breaking_re.is_match("feat: add stuff"));
        assert!(!breaking_re.is_match("major: update without breaking change"));

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
        let breaking_re = Regex::new(BREAKING_REGEX_STR).unwrap();

        for (message, expect_major, expect_minor, expect_patch) in test_cases {
            let to_commit_id = repo
                .commit(Some("HEAD"), &sig, &sig, message, &tree, &[&base_commit])
                .unwrap();
            let to_commit = repo.find_commit(to_commit_id).unwrap();

            let (bump, summary) =
                calculate_version_bump(&repo, &base_commit, &to_commit, &major_re, &minor_re, &noop_re, &breaking_re);

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

            // Check commits vector
            if expect_major || expect_minor || expect_patch {
                assert_eq!(
                    summary.commits.len(),
                    1,
                    "Expected one commit message for: {}",
                    message
                );
                assert_eq!(
                    summary.commits[0].0,
                    to_commit.id().to_string(),
                    "Commit ID mismatch for: {}",
                    message
                );
                assert_eq!(
                    summary.commits[0].1,
                    message,
                    "Commit message mismatch for: {}",
                    message
                );
            } else {
                // For no-op commits, expect the commit to be in summary.commits
                assert_eq!(
                    summary.commits.len(),
                    1,
                    "Expected one commit message for no-op: {}",
                    message
                );
                assert_eq!(
                    summary.commits[0].0,
                    to_commit.id().to_string(),
                    "Commit ID mismatch for no-op: {}",
                    message
                );
                assert_eq!(
                    summary.commits[0].1,
                    message,
                    "Commit message mismatch for no-op: {}",
                    message
                );
            }

            // Reset HEAD for the next test.
            repo.reference("HEAD", base_commit.id(), true, "Reset for next test")
                .unwrap();
        }
    }
}