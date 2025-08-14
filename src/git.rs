use git2::{Commit, Repository};
use crate::error::VNextError;
use url::Url;

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

/// Open the Git repository in the current directory
pub fn open_repository() -> Result<Repository, VNextError> {
    Repository::open(".").map_err(|e| e.into())
}

/// Resolve the HEAD reference to a commit
pub fn resolve_head(repo: &Repository) -> Result<Commit, VNextError> {
    let head_ref = repo.head()?;
    let commit = head_ref.peel_to_commit()?;
    Ok(commit)
}

/// Extract repository information from a git remote URL
/// Returns (host, owner, name) if successful
pub fn extract_repo_info(remote_url: &str) -> Option<(String, String, String)> {
    // Handle SSH URLs like git@github.com:owner/repo.git or git@gitlab.com:owner/repo.git
    if remote_url.starts_with("git@") && remote_url.contains(':') {
        let host_part = remote_url.split('@').nth(1)?.split(':').next()?;
        let path = remote_url.split(':').nth(1)?;
        let path = path.trim_end_matches(".git");
        let parts: Vec<&str> = path.split('/').collect();
        if parts.len() >= 2 {
            return Some((host_part.to_string(), parts[0].to_string(), parts[1].to_string()));
        }
    }
    
    // Handle HTTPS URLs like https://github.com/owner/repo.git or https://gitlab.com/owner/repo.git
    if let Ok(url) = Url::parse(remote_url) {
        let host = url.host_str()?;
        let path = url.path().trim_start_matches('/').trim_end_matches(".git");
        let parts: Vec<&str> = path.split('/').collect();
        if parts.len() >= 2 {
            return Some((host.to_string(), parts[0].to_string(), parts[1].to_string()));
        }
    }
    
    None
}

#[cfg(test)]
mod tests {
    use crate::regex::{MAJOR_REGEX_STR, MINOR_REGEX_STR, NOOP_REGEX_STR, BREAKING_REGEX_STR};
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
        assert!(!breaking_re.is_match("BREAKING CHANGE: this is major")); // Should not match standalone line
        assert!(!breaking_re.is_match("feat: add stuff\nBREAKING CHANGE: old stuff removed")); // Only one newline, not at start of body
        assert!(!breaking_re.is_match("fix: bugfix\nBREAKING CHANGE: changed behavior")); // Only one newline, not at start of body
        assert!(breaking_re.is_match("feat: add stuff\n\nBREAKING CHANGE: old stuff removed")); // Correct format: title, empty line, then BREAKING CHANGE
        assert!(!breaking_re.is_match("feat: add stuff\n\nThis is body.\nBREAKING CHANGE: not first line")); // Not at start of body
        assert!(!breaking_re.is_match("feat: add stuff")); // No breaking change
        assert!(!breaking_re.is_match("major: update without breaking change")); // No breaking change

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