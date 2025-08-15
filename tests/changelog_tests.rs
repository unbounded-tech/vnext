use std::fs;
use std::process::Command;

// Import the test_helpers module
mod test_helpers;
use test_helpers::{run_and_show_command, run_vnext};

#[test]
fn test_changelog() {

    let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");
    let repo_path = temp_dir.path();

    run_and_show_command("git", &["init"], repo_path);
    run_and_show_command("git", &["config", "user.name", "patrickleet"], repo_path);
    run_and_show_command("git", &["config", "user.email", "pat@patscott.io"], repo_path);

    let readme_path = repo_path.join("README.md");
    fs::write(&readme_path, "# Test Repository").expect("Failed to write README file");
    run_and_show_command("git", &["status"], repo_path);
    run_and_show_command("git", &["add", readme_path.to_str().unwrap()], repo_path);
    run_and_show_command("git", &["commit", "-m", "feat: Initial commit"], repo_path);

    let version = run_vnext(repo_path);
    assert_eq!(version, "0.1.0", "Initial version should be 0.1.0");
    println!("Asserted version {} is 0.1.0", version);

    let tag_name = format!("v{}", version);
    run_and_show_command("git", &["tag", &tag_name], repo_path);

    let file_path = repo_path.join("1.md");
    fs::write(&file_path, "# 1").expect("Failed to write README file");
    run_and_show_command("git", &["status"], repo_path);
    run_and_show_command("git", &["add", file_path.to_str().unwrap()], repo_path);
    run_and_show_command("git", &["commit", "-m", "fix: 1"], repo_path);
    
    let file_path = repo_path.join("2.md");
    fs::write(&file_path, "# 2").expect("Failed to write README file");
    run_and_show_command("git", &["status"], repo_path);
    run_and_show_command("git", &["add", file_path.to_str().unwrap()], repo_path);
    run_and_show_command("git", &["commit", "-m", "fix: 2"], repo_path);

    let version = run_vnext(repo_path);
    assert_eq!(version, "0.1.1", "Initial version should be 0.1.1");
    println!("Asserted version {} is 0.1.1", version);

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
    
    let changelog = changelog.trim_end().to_string(); // Remove trailing newlines
    let expected_changelog = format!(
        "### What's changed in v0.1.1\n\n* fix: 1\n\n* fix: 2"
    );

    assert_eq!(
        changelog, expected_changelog,
        "Changelog output should match expected format"
    );

    let tag_name = format!("v{}", version);
    run_and_show_command("git", &["tag", &tag_name], repo_path);
        
    // Add a feature with a breaking change
    let file_path = repo_path.join("breaking.md");
    fs::write(&file_path, "# Breaking Change").expect("Failed to write breaking change file");
    run_and_show_command("git", &["status"], repo_path);
    run_and_show_command("git", &["add", file_path.to_str().unwrap()], repo_path);
    run_and_show_command("git", &["commit", "-m", "feat: add new feature\n\nBREAKING CHANGE: This removes the old API"], repo_path);

    let version = run_vnext(repo_path);
    assert_eq!(version, "1.0.0", "Initial version should be 1.0.0");
    println!("Asserted version {} is 1.0.0", version);

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
    
    let changelog = changelog.trim_end().to_string(); // Remove trailing newlines
    let expected_changelog = format!(
        "### What's changed in v1.0.0\n\n* feat: add new feature\n\n  BREAKING CHANGE: This removes the old API"
    );

    assert_eq!(
        changelog, expected_changelog,
        "Changelog output should match expected format"
    );
}