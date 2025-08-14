//! Changelog generation

use crate::models::commit::CommitSummary;
use crate::models::repo::RepoInfo;
use semver::Version;

/// Output the result of the version calculation
pub fn output_result(
    next_version: &Version,
    summary: &CommitSummary,
    show_changelog: bool,
    no_header_scaling: bool,
    current_version: &Version,
    repo_info: &RepoInfo,
) {
    if show_changelog {
        println!("{}", summary.format_changelog(next_version, no_header_scaling, current_version, repo_info));
    } else {
        println!("{}", next_version);
    }
}

/// Output a fallback result when an error occurs
pub fn output_fallback(show_changelog: bool) {
    if show_changelog {
        println!("## What's changed in 0.0.0\n\n* No changes\n\n---");
    } else {
        println!("0.0.0");
    }
}