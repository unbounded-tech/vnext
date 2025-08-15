//! Version calculation logic

use semver::{BuildMetadata, Prerelease, Version};
use git2::{Commit, Repository};
use regex::Regex;
use crate::models::error::VNextError;
use crate::models::version::VersionBump;
use crate::models::changeset::ChangesetSummary;
use log::debug;

/// Parse a version string into a semver Version
pub fn parse_version(tag: &str) -> Result<Version, semver::Error> {
    let cleaned_tag = tag.trim_start_matches('v');
    Version::parse(cleaned_tag)
}

/// Calculate the next version based on the current version and the version bump
pub fn calculate_next_version(current: &Version, bump: &VersionBump) -> Version {
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
) -> Result<(VersionBump, ChangesetSummary), VNextError> {
    let mut bump = VersionBump { major: false, minor: false, patch: false };
    let mut summary = ChangesetSummary::new();

    // Build a revwalk starting from HEAD.
    let mut revwalk = repo.revwalk()?;
    revwalk.push(to.id())?;

    // If a previous tag exists, hide it so we walk only the newer commits.
    if let Some((_, tag_commit)) = crate::services::git::find_latest_tag(repo) {
        revwalk.hide(tag_commit.id())?;
    }

    // Iterate commits (newest first). We collect and then reverse for changelog display.
    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
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

        summary.commits.push((oid.to_string(), message, None));
    }

    Ok((bump, summary))
}

/// Find the version base (main branch, latest tag, base commit)
pub fn find_version_base<'repo, 'head>(repo: &'repo Repository, head: &'head Commit<'repo>) -> (Version, Commit<'repo>) {
    let main_branch = crate::services::git::find_main_branch(repo).expect("Failed to find main branch");
    debug!("Main branch detected: {}", main_branch);

    let (start_version, last_tag_commit) = match crate::services::git::find_latest_tag(repo) {
        Some((tag, commit)) => {
            let version = parse_version(&tag).unwrap_or_else(|_| Version::new(0, 0, 0));
            debug!("Last release: {} at commit {}", tag, commit.id());
            (version, commit)
        }
        None => {
            debug!("No previous release tags found, starting from 0.0.0");
            let version = Version::new(0, 0, 0);
            
            // Find the initial commit in the repository
            let mut current = head.clone();
            let initial_commit;
            
            // Traverse to the root commit by following the first parent chain
            loop {
                let parents = current.parents();
                if parents.count() == 0 {
                    // We've reached a commit with no parents (the initial commit)
                    initial_commit = current;
                    break;
                }
                
                // Move to the first parent and continue
                current = current.parents().next().unwrap();
            }
            
            debug!("Found initial commit: {}", initial_commit.id());
            (version, initial_commit)
        }
    };
    debug!("Last tag or base commit: {}", last_tag_commit.id());

    // Determine the base commit: use merge base with main if tag exists, otherwise use the initial commit
    let base_commit = if crate::services::git::find_latest_tag(repo).is_some() {
        let merge_base = repo
            .merge_base(head.id(), last_tag_commit.id())
            .expect("Failed to find merge base between HEAD and tag");
        repo.find_commit(merge_base)
            .expect("Failed to find merge base commit")
    } else {
        // When no tags exist, we want to analyze all commits from the initial commit to HEAD
        last_tag_commit.clone()
    };
    debug!("Base commit for analysis: {}", base_commit.id());
    
    (start_version, base_commit)
}

/// Calculate the next version based on commit history
pub fn calculate_version(
    repo: &Repository,
    head: &Commit,
    major_re: &Regex,
    minor_re: &Regex,
    noop_re: &Regex,
    breaking_re: &Regex,
    start_version: &Version,
    base_commit: &Commit,
) -> Result<(Version, ChangesetSummary), VNextError> {
    // Calculate version bump
    let (bump, summary) = calculate_version_bump(
        repo, base_commit, head, major_re, minor_re, noop_re, breaking_re)?;
    
    // Calculate next version
    let next_version = calculate_next_version(&start_version, &bump);
    
    log::debug!(
        "Version bump: major={}, minor={}, patch={}",
        bump.major, bump.minor, bump.patch
    );
    log::debug!("Next version: {}", next_version);
    
    Ok((next_version, summary))
}