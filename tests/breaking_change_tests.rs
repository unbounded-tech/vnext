use std::fs;

// Import the test_helpers module
mod test_helpers;
use test_helpers::{run_and_show_command, run_vnext};

#[test]
fn test_breaking_change_detection() {
    // Create a new temp dir
    let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");
    let repo_path = temp_dir.path();
    println!("Temporary directory created at: {:?}", repo_path);

    // Initialize as git repo
    run_and_show_command("git", &["init"], repo_path);
    run_and_show_command("git", &["config", "user.name", "patrickleet"], repo_path);
    run_and_show_command("git", &["config", "user.email", "pat@patscott.io"], repo_path);

    // 1. Add initial commit
    let file_path = repo_path.join("initial.md");
    fs::write(&file_path, "# Initial commit").expect("Failed to write file");
    run_and_show_command("git", &["add", file_path.to_str().unwrap()], repo_path);
    run_and_show_command("git", &["commit", "-m", "feat: Initial commit"], repo_path);
    
    let version = run_vnext(repo_path);
    assert_eq!(version, "0.1.0", "Initial version should be 0.1.0");
    println!("Asserted version {} is 0.1.0", version);
    
    let tag_name = format!("v{}", version);
    run_and_show_command("git", &["tag", &tag_name], repo_path);

    // 2. Add a commit with BREAKING CHANGE at the start of the first line in the commit body
    // This SHOULD trigger a major version bump
    let file_path = repo_path.join("breaking-at-start-of-first-line.md");
    fs::write(&file_path, "# Breaking change at start of first line").expect("Failed to write file");
    run_and_show_command("git", &["add", file_path.to_str().unwrap()], repo_path);
    run_and_show_command(
        "git",
        &["commit", "-m", "feat: add feature with breaking change\n\nBREAKING CHANGE: This removes the old API"],
        repo_path
    );
    
    let version = run_vnext(repo_path);
    assert_eq!(version, "1.0.0", "Version should be 1.0.0 after breaking change at start of first line");
    println!("Asserted version {} is 1.0.0", version);
    
    let tag_name = format!("v{}", version);
    run_and_show_command("git", &["tag", &tag_name], repo_path);

    // 3. Add a commit with BREAKING CHANGE in the middle of a line in the commit body
    // This should NOT trigger a major version bump
    let file_path = repo_path.join("breaking-in-middle.md");
    fs::write(&file_path, "# Breaking change in middle of line").expect("Failed to write file");
    run_and_show_command("git", &["add", file_path.to_str().unwrap()], repo_path);
    run_and_show_command(
        "git",
        &["commit", "-m", "feat: add another feature\n\nThis line has BREAKING CHANGE: in the middle and should not trigger a major bump."],
        repo_path
    );
    
    let version = run_vnext(repo_path);
    assert_eq!(version, "1.1.0", "Version should be 1.1.0 after BREAKING CHANGE in middle of line");
    println!("Asserted version {} is 1.1.0", version);
    
    let tag_name = format!("v{}", version);
    run_and_show_command("git", &["tag", &tag_name], repo_path);
    
    // 4. Add a commit with BREAKING CHANGE at the start of a line that is NOT the first line of the commit body
    // This should NOT trigger a major version bump
    let file_path = repo_path.join("breaking-not-first-line.md");
    fs::write(&file_path, "# Breaking change not at first line").expect("Failed to write file");
    run_and_show_command("git", &["add", file_path.to_str().unwrap()], repo_path);
    run_and_show_command(
        "git",
        &["commit", "-m", "feat: add another feature\n\nThis is the first line of the commit body.\n\nBREAKING CHANGE: This is not the first line of the body and should not trigger a major bump."],
        repo_path
    );
    
    let version = run_vnext(repo_path);
    assert_eq!(version, "1.2.0", "Version should be 1.2.0 after BREAKING CHANGE not at first line");
    println!("Asserted version {} is 1.2.0", version);
}