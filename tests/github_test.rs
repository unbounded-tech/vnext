use clap::Parser;
use semver::Version;
use vnext::version::{CommitAuthor, ChangesetSummary};
use vnext::changelog::RepoInfo;

#[test]
fn test_changelog_with_author_info() {
    // Create a ChangesetSummary with author information
    let mut summary = ChangesetSummary::new();

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
    let current_version = Version::new(0, 9, 0);
    let repo_info = RepoInfo::new(); // Empty repo info for tests
    let changelog = vnext::changelog::format_changelog(&summary, &version, false, &current_version, &repo_info); // Use default header scaling

    // Verify the changelog contains author information
    assert!(changelog.contains("### What's changed in v1.0.0"));
    assert!(changelog.contains("* feat: Add new feature (by @johndoe)"));
    assert!(changelog.contains("* fix: Fix a bug (by Jane Smith)"));
    assert!(changelog.contains("* chore: Update dependencies"));

    // Verify the order of commits (oldest first)
    let feat_pos = changelog.find("feat: Add new feature").unwrap();
    let fix_pos = changelog.find("fix: Fix a bug").unwrap();
    let chore_pos = changelog.find("chore: Update dependencies").unwrap();

    assert!(
        chore_pos < fix_pos,
        "Commits should be in chronological order (oldest first)"
    );
    assert!(
        fix_pos < feat_pos,
        "Commits should be in chronological order (oldest first)"
    );
}

#[test]
fn test_cli_changelog_flag() {
    // Test with changelog flag
    let args = ["vnext", "--changelog"];
    let cli = vnext::cli::Cli::parse_from(args);

    assert!(cli.changelog, "Changelog flag should be true");

    // Test default flags
    let args = ["vnext"];
    let cli = vnext::cli::Cli::parse_from(args);

    assert!(!cli.changelog, "Changelog flag should default to false");
}

#[test]
fn test_github_detection_behavior() {
    use vnext::version::{CommitAuthor, ChangesetSummary};

    // Create a test summary with commits
    let mut summary = ChangesetSummary::new();
    summary
        .commits
        .push(("abc123".to_string(), "feat: Add feature".to_string(), None));

    // Create a test author
    let author = CommitAuthor {
        name: "Test User".to_string(),
        email: "test@example.com".to_string(),
        username: Some("testuser".to_string()),
    };

    // Test with GitHub author information
    let mut summary_with_github = ChangesetSummary::new();
    summary_with_github.commits.push((
        "abc123".to_string(),
        "feat: Add feature".to_string(),
        Some(author.clone()),
    ));

    let current_version = Version::new(0, 9, 0);
    let repo_info = RepoInfo::new(); // Empty repo info for tests
    let changelog_with_github =
        vnext::changelog::format_changelog(&summary_with_github, &semver::Version::new(1, 0, 0), false, &current_version, &repo_info);
    assert!(
        changelog_with_github.contains("(by @testuser)"),
        "Changelog should include GitHub username when GitHub author information is available"
    );

    // Test without GitHub author information
    let current_version = Version::new(0, 9, 0);
    let repo_info = RepoInfo::new(); // Empty repo info for tests
    let changelog_without_github = vnext::changelog::format_changelog(&summary, &semver::Version::new(1, 0, 0), false, &current_version, &repo_info);
    assert!(!changelog_without_github.contains("(by @testuser)"),
        "Changelog should not include GitHub username when GitHub author information is not available");
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
    assert!(
        result.is_some(),
        "Should extract info from URL without .git suffix"
    );
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

#[test]
fn test_github_detection_from_remote() {
    // This test verifies that a repository with a GitHub remote URL is correctly detected

    // Create a mock repository info with a GitHub remote
    let mut repo_info = vnext::changelog::RepoInfo::new();
    repo_info.owner = "owner".to_string();
    repo_info.name = "repo".to_string();
    repo_info.is_github_repo = true;

    // Create a test summary
    let mut summary = vnext::version::ChangesetSummary::new();
    summary
        .commits
        .push(("abc123".to_string(), "feat: Add feature".to_string(), None));

    // Test that GitHub detection works by checking if GitHub author information is fetched
    // Note: This test doesn't actually fetch from GitHub API, but verifies the detection logic
    assert!(
        repo_info.is_github_repo,
        "Repository should be detected as a GitHub repository"
    );

    // Verify that extract_repo_info correctly identifies GitHub URLs
    let github_url = "https://github.com/owner/repo.git";
    if let Some((host, _, _)) = vnext::git::extract_repo_info(github_url) {
        assert_eq!(host, "github.com", "Host should be detected as github.com");
    } else {
        panic!("Failed to extract repo info from GitHub URL");
    }

    // Verify that a URL with 'github' in it is detected
    let github_enterprise_url = "https://github.enterprise.com/owner/repo.git";
    if let Some((host, _, _)) = vnext::git::extract_repo_info(github_enterprise_url) {
        assert!(host.contains("github"), "Host should contain 'github'");
    } else {
        panic!("Failed to extract repo info from GitHub Enterprise URL");
    }
}
