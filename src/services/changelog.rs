//! Changelog generation

use crate::models::changeset::ChangesetSummary;
use crate::models::repo::RepoInfo;
use semver::Version;

/// Format a changelog from a commit summary
///
/// This function generates a formatted changelog based on the provided commit summary,
/// version information, and repository details.
pub fn format_changelog(
    summary: &ChangesetSummary,
    next_version: &Version,
    no_header_scaling: bool,
    current_version: &Version,
    repo_info: &RepoInfo,
) -> String {
    let mut changelog = format!("### What's changed in v{}\n\n", next_version);
    if summary.commits.is_empty() {
        changelog.push_str("* No changes\n");
    } else {
        // Reverse the commits to display them in chronological order (oldest first)
        let mut commits = summary.commits.clone();
        commits.reverse();
        for commit in &commits {
            // Format the first line with the commit title
            let first_line = if let Some(scope) = &commit.scope {
                format!("* {}({}): {}", commit.commit_type, scope, commit.title)
            } else {
                format!("* {}: {}", commit.commit_type, commit.title)
            };
            
            // Add author information if available
            let line_with_author = if let Some(author_info) = &commit.author {
                if let Some(username) = &author_info.username {
                    format!("{} (by @{})\n", first_line, username)
                } else {
                    format!("{} (by {})\n", first_line, author_info.name)
                }
            } else {
                format!("{}\n", first_line)
            };
            
            changelog.push_str(&line_with_author);
            
            // Add the commit body if present
            if let Some(body) = &commit.body {
                // If this is a breaking change body, add the prefix back
                let body_with_prefix = if commit.breaking_change_body {
                    format!("BREAKING CHANGE: {}", body)
                } else {
                    body.clone()
                };
                
                // Format the body with proper indentation and header scaling
                let formatted_body = format_commit_body(&body_with_prefix, no_header_scaling);
                if !formatted_body.is_empty() {
                    // Add a single newline before the body
                    changelog.push_str("\n");
                    changelog.push_str(&formatted_body);
                    changelog.push('\n');
                }
            }

            changelog.push_str("\n");
        }
    }
    
    // Add comparison link if it's a GitHub repository and current version is not 0.0.0
    if repo_info.is_github_repo && (current_version.major > 0 || current_version.minor > 0 || current_version.patch > 0) {
        changelog.push('\n');
        changelog.push_str(&format!("See full diff: [v{}...v{}](https://github.com/{}/{}/compare/v{}...v{})",
            current_version, next_version, repo_info.owner, repo_info.name, current_version, next_version));
    }
    
    changelog
}

/// Format a commit body with proper indentation and header scaling
fn format_commit_body(body: &str, no_header_scaling: bool) -> String {
    let mut formatted = String::new();
    let lines: Vec<&str> = body.lines().collect();
    
    // Process each line
    for (i, line) in lines.iter().enumerate() {
        if line.is_empty() {
            // Only add a newline for empty lines if it's not the first line
            if i > 0 {
                formatted.push('\n');
            }
        } else {
            let processed_line = if !no_header_scaling {
                // Scale down headers in commit body (h1->h4, h2->h5, h3->h6)
                if line.starts_with("# ") {
                    format!("#### {}", &line[2..])
                } else if line.starts_with("## ") {
                    format!("##### {}", &line[3..])
                } else if line.starts_with("### ") {
                    format!("###### {}", &line[4..])
                } else {
                    line.to_string()
                }
            } else {
                // No header scaling
                line.to_string()
            };
            formatted.push_str(&format!("  {}\n", processed_line));
        }
    }
    
    // Remove trailing newlines to match expected format
    formatted.trim_end().to_string()
}

/// Output the result of the version calculation
pub fn output_result(
    next_version: &Version,
    summary: &ChangesetSummary,
    show_changelog: bool,
    no_header_scaling: bool,
    current_version: &Version,
    repo_info: &RepoInfo,
) {
    if show_changelog {
        println!("{}", format_changelog(summary, next_version, no_header_scaling, current_version, repo_info));
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