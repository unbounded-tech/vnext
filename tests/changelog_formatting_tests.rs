use std::fs;
use std::process::Command;

// Import the test_helpers module
mod test_helpers;
use test_helpers::{run_and_show_command, run_vnext};

#[test]
fn test_changelog_formatting() {
    // Create a new temp dir
    let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");
    let repo_path = temp_dir.path();
    println!("Temporary directory created at: {:?}", repo_path);

    // Initialize as git repo
    run_and_show_command("git", &["init"], repo_path);
    run_and_show_command("git", &["config", "user.name", "patrickleet"], repo_path);
    run_and_show_command("git", &["config", "user.email", "pat@patscott.io"], repo_path);

    // 1. Add a commit with a simple multi-line body
    let file_path = repo_path.join("simple-body.md");
    fs::write(&file_path, "# Simple body commit").expect("Failed to write file");
    run_and_show_command("git", &["add", file_path.to_str().unwrap()], repo_path);
    run_and_show_command(
        "git",
        &["commit", "-m", "feat: add feature with simple body\n\nThis is a simple commit body\nIt has multiple lines\nThey should be indented"],
        repo_path
    );

    // 2. Add a commit with a body that includes empty lines
    let file_path = repo_path.join("empty-lines.md");
    fs::write(&file_path, "# Empty lines commit").expect("Failed to write file");
    run_and_show_command("git", &["add", file_path.to_str().unwrap()], repo_path);
    run_and_show_command(
        "git",
        &["commit", "-m", "fix: fix bug with empty lines in body\n\nThis commit body has empty lines\n\nLike this one above\n\nAnd this one too"],
        repo_path
    );

    // 3. Add a commit that resembles a squashed PR from GitHub UI
    let file_path = repo_path.join("squashed-pr.md");
    fs::write(&file_path, "# Squashed PR commit").expect("Failed to write file");
    run_and_show_command("git", &["add", file_path.to_str().unwrap()], repo_path);
    run_and_show_command(
        "git",
        &["commit", "-m", "feat: add feature from squashed PR\n\nThis is a squashed commit that includes the following changes:\n\n- feat(core): create plugin interface\n\n- feat(core): implement plugin loader\n\n- fix(core): handle plugin initialization errors\n\n- test(core): add tests for plugin system"],
        repo_path
    );

    // Run vnext and check version
    let version = run_vnext(repo_path);
    assert_eq!(version, "0.1.0", "First run version should be 0.1.0");
    println!("Asserted version {} is 0.1.0", version);

    // Check changelog
    println!("Running vnext with --changelog to verify changelog output");
    let project_dir = std::env::current_dir().expect("Failed to get current directory");
    let binary_path = project_dir.join("target/debug/vnext");
    let output = Command::new(&binary_path)
        .args(["--changelog"])
        .current_dir(&repo_path)
        .output()
        .expect("Failed to execute vnext with --changelog");
    
    let changelog = String::from_utf8_lossy(&output.stdout).to_string();
    println!("Changelog output:\n{}", changelog);
    
    // Verify changelog formatting
    let changelog = changelog.trim_end().to_string();
    
    // Check that the version header is present
    assert!(changelog.contains("### What's changed in v0.1.0"), "Changelog should mention version 0.1.0");
    
    // Check that all commit titles are present
    assert!(changelog.contains("* feat: add feature with simple body"), "Changelog should contain first commit title");
    assert!(changelog.contains("* fix: fix bug with empty lines in body"), "Changelog should contain second commit title");
    assert!(changelog.contains("* feat: add feature from squashed PR"), "Changelog should contain third commit title");
    
    // Check that commit bodies are properly indented
    assert!(changelog.contains("  This is a simple commit body"), "Commit body should be indented");
    assert!(changelog.contains("  It has multiple lines"), "Commit body should be indented");
    assert!(changelog.contains("  They should be indented"), "Commit body should be indented");
    
    // Check that empty lines in commit bodies are preserved
    assert!(changelog.contains("  This commit body has empty lines"), "Commit body should be indented");
    assert!(changelog.contains("\n\n  Like this one above"), "Empty line should be preserved");
    assert!(changelog.contains("\n\n  And this one too"), "Empty line should be preserved");
    
    // Check that squashed PR format is properly indented
    assert!(changelog.contains("  This is a squashed commit that includes the following changes:"), "Squashed PR intro should be indented");
    assert!(changelog.contains("  - feat(core): create plugin interface"), "Squashed PR bullet point should be indented");
    assert!(changelog.contains("  - feat(core): implement plugin loader"), "Squashed PR bullet point should be indented");
    assert!(changelog.contains("  - fix(core): handle plugin initialization errors"), "Squashed PR bullet point should be indented");
    assert!(changelog.contains("  - test(core): add tests for plugin system"), "Squashed PR bullet point should be indented");
    
    // Tag the version
    let tag_name = format!("v{}", version);
    run_and_show_command("git", &["tag", &tag_name], repo_path);
}