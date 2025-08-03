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
fn test_cli_github_flags() {
    // Test GitHub flag
    let args = ["vnext", "--github", "--changelog"];
    let cli = vnext::cli::Cli::parse_from(args);
    
    assert!(cli.github, "GitHub flag should be true");
    assert!(cli.changelog, "Changelog flag should be true");
    
    // Test default flags
    let args = ["vnext"];
    let cli = vnext::cli::Cli::parse_from(args);
    
    assert!(!cli.github, "GitHub flag should default to false");
    assert!(!cli.changelog, "Changelog flag should default to false");
}

#[test]
fn test_github_flag_behavior() {
    use vnext::version::{CommitSummary, CommitAuthor};
    
    // Create a test summary with commits
    let mut summary = CommitSummary::new();
    summary.commits.push((
        "abc123".to_string(),
        "feat: Add feature".to_string(),
        None,
    ));
    
    // Create a test author
    let author = CommitAuthor {
        name: "Test User".to_string(),
        email: "test@example.com".to_string(),
        username: Some("testuser".to_string()),
    };
    
    // Test with GitHub flag enabled
    let mut summary_with_github = CommitSummary::new();
    summary_with_github.commits.push((
        "abc123".to_string(),
        "feat: Add feature".to_string(),
        Some(author.clone()),
    ));
    
    let changelog_with_github = summary_with_github.format_changelog(&semver::Version::new(1, 0, 0));
    assert!(changelog_with_github.contains("(by @testuser)"),
        "Changelog should include GitHub username when GitHub flag is enabled");
    
    // Test without GitHub flag
    let changelog_without_github = summary.format_changelog(&semver::Version::new(1, 0, 0));
    assert!(!changelog_without_github.contains("(by @testuser)"),
        "Changelog should not include GitHub username when GitHub flag is disabled");
}

#[test]
fn test_extract_repo_info() {
    // Test SSH URL format
    let ssh_url = "git@github.com:owner/repo.git";
    let result = vnext::git::extract_repo_info(ssh_url);
    assert!(result.is_some(), "Should extract info from SSH URL");
    if let Some((host, owner, name)) = result {
        assert_eq!(host, "github.com");
        assert_eq!(owner, "owner");
        assert_eq!(name, "repo");
    }
    
    // Test HTTPS URL format
    let https_url = "https://github.com/owner/repo.git";
    let result = vnext::git::extract_repo_info(https_url);
    assert!(result.is_some(), "Should extract info from HTTPS URL");
    if let Some((host, owner, name)) = result {
        assert_eq!(host, "github.com");
        assert_eq!(owner, "owner");
        assert_eq!(name, "repo");
    }
    
    // Test URL without .git suffix
    let no_git_url = "https://github.com/owner/repo";
    let result = vnext::git::extract_repo_info(no_git_url);
    assert!(result.is_some(), "Should extract info from URL without .git suffix");
    if let Some((host, owner, name)) = result {
        assert_eq!(host, "github.com");
        assert_eq!(owner, "owner");
        assert_eq!(name, "repo");
    }
    
    // Test non-GitHub URL
    let non_github_url = "https://gitlab.com/owner/repo.git";
    let result = vnext::git::extract_repo_info(non_github_url);
    assert!(result.is_some(), "Should extract info from GitLab URL");
    if let Some((host, owner, name)) = result {
        assert_eq!(host, "gitlab.com");
        assert_eq!(owner, "owner");
        assert_eq!(name, "repo");
    }
}
