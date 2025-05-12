use git2::{Commit, Repository};
use regex::Regex;
use crate::version::{CommitSummary, VersionBump};

/// Find the main branch ("main" or "master").
pub fn find_main_branch(repo: &Repository) -> Option<String> {
    for branch in ["main", "master"] {
        if repo.find_branch(branch, git2::BranchType::Local).is_ok() {
            return Some(branch.to_string());
        }
    }
    None
}

/// Find the latest semver tag in the repo, returning (tag_name, commit).
pub fn find_latest_tag(repo: &Repository) -> Option<(String, Commit)> {
    let tags = repo.tag_names(None).expect("Failed to get tag names");
    let mut latest: Option<(String, Commit)> = None;
    let mut max_version = crate::version::parse_version("0.0.0").unwrap();

    for tag in tags.iter().flatten() {
        if let Ok(reference) = repo.find_reference(&format!("refs/tags/{}", tag)) {
            if let Ok(commit) = reference.peel_to_commit() {
                if let Ok(version) = crate::version::parse_version(tag) {
                    if version > max_version {
                        max_version = version;
                        latest = Some((tag.to_string(), commit));
                    }
                }
            }
        }
    }
    latest
}

/// Calculate how the version should bump between `from` and `to` commits.
/// Uses a revwalk to include or exclude the base commit as appropriate.
pub fn calculate_version_bump(
    repo: &Repository,
    _from: &Commit,
    to: &Commit,
    major_re: &Regex,
    minor_re: &Regex,
    noop_re: &Regex,
    breaking_re: &Regex,
) -> (VersionBump, CommitSummary) {
    let mut bump = VersionBump { major: false, minor: false, patch: false };
    let mut summary = CommitSummary::new();

    // Build a revwalk starting from HEAD.
    let mut revwalk = repo.revwalk().expect("Failed to create revwalk");
    revwalk.push(to.id()).expect("Failed to push HEAD to revwalk");

    // If a previous tag exists, hide it so we walk only the newer commits.
    if let Some((_, tag_commit)) = find_latest_tag(repo) {
        revwalk.hide(tag_commit.id()).expect("Failed to hide tag commit");
    }

    // Iterate commits (newest first). We collect and then reverse for changelog display.
    for oid in revwalk {
        let oid = oid.expect("Invalid OID in revwalk");
        let commit = repo.find_commit(oid).expect("Failed to find commit");
        let message = commit.message().unwrap_or("").to_string();

        // Decide bump level
        if breaking_re.is_match(&message) || major_re.is_match(&message) {
            bump.major = true;
            summary.major += 1;
        } else if minor_re.is_match(&message) {
            bump.minor = true;
            summary.minor += 1;
        } else if !noop_re.is_match(&message) {
            bump.patch = true;
            summary.patch += 1;
        } else {
            summary.noop += 1;
        }

        summary.commits.push((oid.to_string(), message));
    }

    (bump, summary)
}

#[cfg(test)]
mod tests {
    use crate::constants::{MAJOR_REGEX_STR, MINOR_REGEX_STR, NOOP_REGEX_STR, BREAKING_REGEX_STR};
    use regex::Regex;

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
}