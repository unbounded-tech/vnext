use std::fs;
use std::process::Command;

// Import the test_helpers module
mod test_helpers;
use test_helpers::{run_and_show_command, run_vnext};

#[test]
fn version_tests() {

    // 1. Run vnext on empty directory
    print!("Running vnext in empty directory");
    let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");
    let repo_path = temp_dir.path();
    println!("Temporary directory created at: {:?}", repo_path);
    let version = run_vnext(repo_path);
    assert_eq!(version, "0.0.0", "Version should be 0.0.0 on empty repo");
    println!("Asserted version {} is 0.0.0", version);

    // 2. Initialize the directory as a git repository  
    println!("Initializing git repository in temporary directory, and running vnext again");  
    run_and_show_command("git", &["init"], repo_path);
    run_and_show_command("git", &["config", "user.name", "patrickleet"], repo_path);
    run_and_show_command("git", &["config", "user.email", "pat@patscott.io"], repo_path);

    let version = run_vnext(repo_path);
    assert_eq!(version, "0.0.0", "Version should still be 0.0.0 after git init");
    println!("Asserted version {} is still 0.0.0", version);
    
    // 3. Create a README file and commit it
    println!("Creating README file and committing it, then running vnext again");
    let readme_path = repo_path.join("README.md");
    fs::write(&readme_path, "# Test Repository").expect("Failed to write README file");
    println!("Created README file at: {:?}", readme_path);

    run_and_show_command("git", &["status"], repo_path);
    run_and_show_command("git", &["add", readme_path.to_str().unwrap()], repo_path);
    run_and_show_command("git", &["commit", "-m", "feat: Initial commit"], repo_path);
    
    let version = run_vnext(repo_path);
    assert_eq!(version, "0.1.0", "Initial version should be 0.1.0");
    println!("Asserted version {} is 0.1.0", version);

    // Add tag for the initial version
    let tag_name = format!("v{}", version);
    run_and_show_command("git", &["tag", &tag_name], repo_path);

    // 4. Add a file "patch", and a patch commit
    println!("Creating patch file and committing it, then running vnext again");
    let patch_path = repo_path.join("patch");
    fs::write(&patch_path, "This is a patch").expect("Failed to write patch file");
    println!("Created patch file at: {:?}", patch_path);
    run_and_show_command("git", &["status"], repo_path);
    run_and_show_command("git", &["add", patch_path.to_str().unwrap()], repo_path);
    run_and_show_command("git", &["commit", "-m", "fix: Bug fix"], repo_path);

    let version = run_vnext(repo_path);
    assert_eq!(version, "0.1.1", "Patch version should be 0.1.1");
    println!("Asserted version {} is 0.1.1", version);
    
    let tag_name = format!("v{}", version);
    run_and_show_command("git", &["tag", &tag_name], repo_path);

    // 5. add another patch commit
    println!("Creating another patch file and committing it, then running vnext again");
    let patch_path = repo_path.join("patch2");
    fs::write(&patch_path, "This is another patch").expect("Failed to write patch file");
    println!("Created patch file at: {:?}", patch_path);
    run_and_show_command("git", &["status"], repo_path);
    run_and_show_command("git", &["add", patch_path.to_str().unwrap()], repo_path);
    run_and_show_command("git", &["commit", "-m", "fix: Another bug fix"], repo_path);
    let version = run_vnext(repo_path);
    assert_eq!(version, "0.1.2", "Patch version should be 0.1.2");
    println!("Asserted version {} is 0.1.2", version);
    let tag_name = format!("v{}", version);
    run_and_show_command("git", &["tag", &tag_name], repo_path);

    // 6. add a feature commit
    println!("Creating feature file and committing it, then running vnext again");
    let feature_path = repo_path.join("feature");
    fs::write(&feature_path, "This is a feature").expect("Failed to write feature file");
    println!("Created feature file at: {:?}", feature_path);
    run_and_show_command("git", &["status"], repo_path);
    run_and_show_command("git", &["add", feature_path.to_str().unwrap()], repo_path);
    run_and_show_command("git", &["commit", "-m", "feat: New feature"], repo_path);
    let version = run_vnext(repo_path);
    assert_eq!(version, "0.2.0", "Feature version should be 0.2.0");
    println!("Asserted version {} is 0.2.0", version);
    let tag_name = format!("v{}", version);
    run_and_show_command("git", &["tag", &tag_name], repo_path);

    // 7. add another feature commit with a breaking change
    println!("Creating breaking change file and committing it, then running vnext again");
    let breaking_path = repo_path.join("breaking");
    fs::write(&breaking_path, "This is a breaking change").expect("Failed to write breaking change file");
    println!("Created breaking change file at: {:?}", breaking_path);
    run_and_show_command("git", &["status"], repo_path);
    run_and_show_command("git", &["add", breaking_path.to_str().unwrap()], repo_path);
    run_and_show_command("git", &["commit", "-m", "feat: new stuff \n\nBREAKING CHANGE: old stuff removed"], repo_path);
    let version = run_vnext(repo_path);
    assert_eq!(version, "1.0.0", "Breaking change version should be 1.0.0");
    println!("Asserted version {} is 1.0.0", version);
    let tag_name = format!("v{}", version);
    run_and_show_command("git", &["tag", &tag_name], repo_path);

    // 8. add a "major" commit
    println!("Creating major change file and committing it, then running vnext again");
    let major_path = repo_path.join("major");
    fs::write(&major_path, "This is a major change").expect("Failed to write major change file");
    println!("Created major change file at: {:?}", major_path);
    run_and_show_command("git", &["status"], repo_path);
    run_and_show_command("git", &["add", major_path.to_str().unwrap()], repo_path);
    run_and_show_command("git", &["commit", "-m", "major: v2"], repo_path);
    let version = run_vnext(repo_path);
    assert_eq!(version, "2.0.0", "Major version should be 2.0.0");
    println!("Asserted version {} is 2.0.0", version);
    let tag_name = format!("v{}", version);
    run_and_show_command("git", &["tag", &tag_name], repo_path);

    // 9. add a "minor" commit
    println!("Creating minor change file and committing it, then running vnext again");
    let minor_path = repo_path.join("minor");
    fs::write(&minor_path, "This is a minor change").expect("Failed to write minor change file");
    println!("Created minor change file at: {:?}", minor_path);
    run_and_show_command("git", &["status"], repo_path);
    run_and_show_command("git", &["add", minor_path.to_str().unwrap()], repo_path);
    run_and_show_command("git", &["commit", "-m", "minor: bump"], repo_path);
    let version = run_vnext(repo_path);
    assert_eq!(version, "2.1.0", "Minor version should be 2.1.0");
    println!("Asserted version {} is 2.1.0", version);
    let tag_name = format!("v{}", version);
    run_and_show_command("git", &["tag", &tag_name], repo_path);

    // 10. add a noop commit
    println!("Creating noop change file and committing it, then running vnext again");
    let noop_path = repo_path.join("noop");
    fs::write(&noop_path, "This is a noop change").expect("Failed to write noop change file");
    println!("Created noop change file at: {:?}", noop_path);
    run_and_show_command("git", &["status"], repo_path);
    run_and_show_command("git", &["add", noop_path.to_str().unwrap()], repo_path);
    run_and_show_command("git", &["commit", "-m", "chore: noop"], repo_path);
    let version = run_vnext(repo_path);
    assert_eq!(version, "2.1.0", "Noop version should be 2.1.0");
    println!("Asserted version {} is 2.1.0", version);

    // 11. add a chore commit, also no-op
    println!("Creating chore change file and committing it, then running vnext again");
    let chore_path = repo_path.join("chore");
    fs::write(&chore_path, "This is a chore change").expect("Failed to write chore change file");
    println!("Created chore change file at: {:?}", chore_path);
    run_and_show_command("git", &["status"], repo_path);
    run_and_show_command("git", &["add", chore_path.to_str().unwrap()], repo_path);
    run_and_show_command("git", &["commit", "-m", "chore: noop"], repo_path);
    let version = run_vnext(repo_path);
    assert_eq!(version, "2.1.0", "Chore version should be 2.1.0");
    println!("Asserted version {} is 2.1.0", version);

    // 12. add a commit that does not follow conventional commits, it should result in a patch version bump
    println!("Creating non-conventional change file and committing it, then running vnext again");
    let non_conventional_path = repo_path.join("non-conventional");
    fs::write(&non_conventional_path, "This is a non-conventional change").expect("Failed to write non-conventional change file");
    println!("Created non-conventional change file at: {:?}", non_conventional_path);
    run_and_show_command("git", &["status"], repo_path);
    run_and_show_command("git", &["add", non_conventional_path.to_str().unwrap()], repo_path);
    run_and_show_command("git", &["commit", "-m", "non-conventional: bump"], repo_path);
    let version = run_vnext(repo_path);
    assert_eq!(version, "2.1.1", "Non-conventional version should be 2.1.1");
    println!("Asserted version {} is 2.1.1", version);
    
    // 13. Test changelog output for the last version
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
        "### What's changed in v2.1.1\n\n* chore: noop\n\n* chore: noop\n\n* non-conventional: bump"
    );
    assert_eq!(
        changelog, expected_changelog,
        "Changelog output should match expected format for version 2.1.1"
    );

    let tag_name = format!("v{}", version);
    run_and_show_command("git", &["tag", &tag_name], repo_path);
}