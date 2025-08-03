//! Core functionality for the vnext version calculation

use git2::{Commit, Repository};
use log::debug;
use regex::Regex;
use semver::Version;

use crate::error::VNextError;
use crate::git;
use crate::version::{CommitSummary};
use crate::version;

/// Find the version base (main branch, latest tag, base commit)
pub fn find_version_base<'repo, 'head>(repo: &'repo Repository, head: &'head Commit<'repo>) -> (Version, Commit<'repo>) {
    let main_branch = git::find_main_branch(repo).expect("Failed to find main branch");
    debug!("Main branch detected: {}", main_branch);

    let (start_version, last_tag_commit) = match git::find_latest_tag(repo) {
        Some((tag, commit)) => {
            let version = version::parse_version(&tag).unwrap_or_else(|_| Version::new(0, 0, 0));
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
    let base_commit = if git::find_latest_tag(repo).is_some() {
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
) -> Result<(Version, CommitSummary), VNextError> {
    // Find the version base
    let (start_version, base_commit) = find_version_base(repo, head);
    
    // Calculate version bump
    let (bump, summary) = git::calculate_version_bump(
        repo, &base_commit, head, major_re, minor_re, noop_re, breaking_re)?;
    
    // Calculate next version
    let next_version = version::calculate_next_version(&start_version, &bump);
    
    log::debug!(
        "Version bump: major={}, minor={}, patch={}",
        bump.major, bump.minor, bump.patch
    );
    log::debug!("Next version: {}", next_version);
    
    Ok((next_version, summary))
}