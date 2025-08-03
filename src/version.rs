use semver::{BuildMetadata, Prerelease, Version};

pub struct VersionBump {
    pub major: bool,
    pub minor: bool,
    pub patch: bool,
}

pub struct CommitSummary {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub noop: u32,
    pub commits: Vec<(String, String, Option<CommitAuthor>)>, // (commit_id, message, author)
}

#[derive(Clone, Debug)]
pub struct CommitAuthor {
    pub name: String,
    #[allow(dead_code)]
    pub email: String,
    pub username: Option<String>,
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

    pub fn format_changelog(&self, next_version: &Version) -> String {
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
                                    changelog.push_str(&format!("  {}\n", line));
                                }
                            }
                        }
                    }
                }
            }
        }
        changelog
    }
}

pub fn parse_version(tag: &str) -> Result<Version, semver::Error> {
    let cleaned_tag = tag.trim_start_matches('v');
    Version::parse(cleaned_tag)
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use semver::Version;

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
}