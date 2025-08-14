//! Commit-related data structures

use semver::Version;
use crate::models::repo::RepoInfo;

/// Represents a commit author
#[derive(Clone, Debug)]
pub struct CommitAuthor {
    pub name: String,
    #[allow(dead_code)]
    pub email: String,
    pub username: Option<String>,
}

/// Represents a summary of commits for version calculation
pub struct CommitSummary {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub noop: u32,
    pub commits: Vec<(String, String, Option<CommitAuthor>)>, // (commit_id, message, author)
}

impl CommitSummary {
    pub fn new() -> Self {
        CommitSummary {
            major: 0,
            minor: 0,
            patch: 0,
            noop: 0,
            commits: Vec::new(),
        }
    }

    pub fn format_changelog(&self, next_version: &Version, no_header_scaling: bool, current_version: &Version, repo_info: &RepoInfo) -> String {
        let mut changelog = format!("### What's changed in v{}\n\n", next_version);
        if self.commits.is_empty() {
            changelog.push_str("* No changes\n");
        } else {
            // Reverse the commits to display them in chronological order (oldest first)
            let mut commits = self.commits.clone();
            commits.reverse();
            for (_, message, author) in &commits {
                // Format the message to preserve newlines but add bullet point to first line
                let mut lines = message.lines();
                if let Some(first_line) = lines.next() {
                    // Add author information if available
                    let line_with_author = if let Some(author_info) = author {
                        if let Some(username) = &author_info.username {
                            format!("* {} (by @{})\n", first_line, username)
                        } else {
                            format!("* {} (by {})\n", first_line, author_info.name)
                        }
                    } else {
                        format!("* {}\n", first_line)
                    };
                    
                    changelog.push_str(&line_with_author);
                    
                    // Add any remaining lines with proper indentation
                    let remaining_lines: Vec<&str> = lines.collect();
                    
                    // If there are remaining lines, add an empty line and then the indented body
                    if !remaining_lines.is_empty() {
                        // Skip leading empty lines
                        let mut start_index = 0;
                        while start_index < remaining_lines.len() && remaining_lines[start_index].is_empty() {
                            start_index += 1;
                        }
                        
                        if start_index < remaining_lines.len() {
                            changelog.push('\n');
                            
                            for line in &remaining_lines[start_index..] {
                                if line.is_empty() {
                                    changelog.push('\n');
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
                                    changelog.push_str(&format!("  {}\n", processed_line));
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Add comparison link if it's a GitHub repository and current version is not 0.0.0
        if repo_info.is_github_repo && (current_version.major > 0 || current_version.minor > 0 || current_version.patch > 0) {
            changelog.push_str("\n\n");
            changelog.push_str(&format!("See full diff: [v{}...v{}](https://github.com/{}/{}/compare/v{}...v{})",
                current_version, next_version, repo_info.owner, repo_info.name, current_version, next_version));
        }
        
        changelog
    }
}