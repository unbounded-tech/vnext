use std::fs;
use std::process::Command;

// Import the test_helpers module
mod test_helpers;
use test_helpers::{run_and_show_command, run_vnext};

#[test]
fn test_first_run_scenarios() {
    // Create a new temp dir
    let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");
    let repo_path = temp_dir.path();
    println!("Temporary directory created at: {:?}", repo_path);

    // Initialize as git repo
    run_and_show_command("git", &["init"], repo_path);
    run_and_show_command("git", &["config", "user.name", "patrickleet"], repo_path);
    run_and_show_command("git", &["config", "user.email", "pat@patscott.io"], repo_path);

    // Add files with different commit types
    // 1. fix commit
    let file_path = repo_path.join("1.md");
    fs::write(&file_path, "# First commit").expect("Failed to write 1.md file");
    run_and_show_command("git", &["add", file_path.to_str().unwrap()], repo_path);
    run_and_show_command("git", &["commit", "-m", "fix: first bugfix"], repo_path);

    // 2. feat commit
    let file_path = repo_path.join("2.md");
    fs::write(&file_path, "# Second commit").expect("Failed to write 2.md file");
    run_and_show_command("git", &["add", file_path.to_str().unwrap()], repo_path);
    run_and_show_command("git", &["commit", "-m", "feat: new feature"], repo_path);

    // 3. fix(scope) commit
    let file_path = repo_path.join("3.md");
    fs::write(&file_path, "# Third commit").expect("Failed to write 3.md file");
    run_and_show_command("git", &["add", file_path.to_str().unwrap()], repo_path);
    run_and_show_command("git", &["commit", "-m", "fix(core): scoped bugfix"], repo_path);

    // 4. feat(scope) commit
    let file_path = repo_path.join("4.md");
    fs::write(&file_path, "# Fourth commit").expect("Failed to write 4.md file");
    run_and_show_command("git", &["add", file_path.to_str().unwrap()], repo_path);
    run_and_show_command("git", &["commit", "-m", "feat(ui): scoped feature"], repo_path);

    // 5. noop commit
    let file_path = repo_path.join("5.md");
    fs::write(&file_path, "# Fifth commit").expect("Failed to write 5.md file");
    run_and_show_command("git", &["add", file_path.to_str().unwrap()], repo_path);
    run_and_show_command("git", &["commit", "-m", "noop: no version change"], repo_path);

    // 6. chore commit
    let file_path = repo_path.join("6.md");
    fs::write(&file_path, "# Sixth commit").expect("Failed to write 6.md file");
    run_and_show_command("git", &["add", file_path.to_str().unwrap()], repo_path);
    run_and_show_command("git", &["commit", "-m", "chore: cleanup"], repo_path);

    // 7. feat commit
    let file_path = repo_path.join("7.md");
    fs::write(&file_path, "# Seventh commit").expect("Failed to write 7.md file");
    run_and_show_command("git", &["add", file_path.to_str().unwrap()], repo_path);
    run_and_show_command("git", &["commit", "-m", "feat: another feature"], repo_path);

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
    
    // Verify changelog contains all commits (without being too specific about formatting)
    let changelog = changelog.trim_end().to_string();
    assert!(changelog.contains("What's changed in v0.1.0"), "Changelog should mention version 0.1.0");
    assert!(changelog.contains("fix: first bugfix"), "Changelog should contain first commit");
    assert!(changelog.contains("feat: new feature"), "Changelog should contain new feature");
    assert!(changelog.contains("fix(core): scoped bugfix"), "Changelog should contain scoped bugfix");
    assert!(changelog.contains("feat(ui): scoped feature"), "Changelog should contain scoped feature");
    assert!(changelog.contains("noop: no version change"), "Changelog should contain no-op commit");
    assert!(changelog.contains("chore: cleanup"), "Changelog should contain chore commit");
    assert!(changelog.contains("feat: another feature"), "Changelog should contain another feature");
    
    // Tag the version
    let tag_name = format!("v{}", version);
    run_and_show_command("git", &["tag", &tag_name], repo_path);
}