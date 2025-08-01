use semver::Version;
use vnext::version::{CommitAuthor, CommitSummary};
use clap::Parser;

#[test]
fn test_changelog_with_author_info() {
    // Create a CommitSummary with author information
    let mut summary = CommitSummary::new();
    
    // Add a commit with author information
    let author1 = CommitAuthor {
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        username: Some("johndoe".to_string()),
    };
    summary.commits.push((
        "abc123".to_string(),
        "feat: Add new feature".to_string(),
        Some(author1),
    ));
    
    // Add a commit with author information but no username
    let author2 = CommitAuthor {
        name: "Jane Smith".to_string(),
        email: "jane@example.com".to_string(),
        username: None,
    };
    summary.commits.push((
        "def456".to_string(),
        "fix: Fix a bug".to_string(),
        Some(author2),
    ));
    
    // Add a commit without author information
    summary.commits.push((
        "ghi789".to_string(),
        "chore: Update dependencies".to_string(),
        None,
    ));
    
    // Format the changelog
    let version = Version::new(1, 0, 0);
    let changelog = summary.format_changelog(&version);
    
    // Verify the changelog contains author information
    assert!(changelog.contains("### What's changed in v1.0.0"));
    assert!(changelog.contains("* feat: Add new feature (by @johndoe)"));
    assert!(changelog.contains("* fix: Fix a bug (by Jane Smith)"));
    assert!(changelog.contains("* chore: Update dependencies"));
    
    // Verify the order of commits (oldest first)
    let feat_pos = changelog.find("feat: Add new feature").unwrap();
    let fix_pos = changelog.find("fix: Fix a bug").unwrap();
    let chore_pos = changelog.find("chore: Update dependencies").unwrap();
    
    assert!(chore_pos < fix_pos, "Commits should be in chronological order (oldest first)");
    assert!(fix_pos < feat_pos, "Commits should be in chronological order (oldest first)");
}

#[test]
fn test_cli_github_flag() {
    // This test verifies that the GitHub flag is properly parsed by the CLI
    let args = ["vnext", "--github", "--changelog"];
    let cli = vnext::cli::Cli::parse_from(args);
    
    assert!(cli.github, "GitHub flag should be true");
    assert!(cli.changelog, "Changelog flag should be true");
}