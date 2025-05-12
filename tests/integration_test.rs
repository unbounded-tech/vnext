use std::fs;
use std::path::Path;
use std::process::{Command, Output};

// Helper function to run a command and return its output
fn run_command(cmd: &str, args: &[&str], dir: &Path) -> Output {
    Command::new(cmd)
        .args(args)
        .current_dir(dir)
        .output()
        .expect(&format!("Failed to execute command: {} {:?}", cmd, args))
}

// Helper function to run a command, print it, and show its output
fn run_and_show_command(cmd: &str, args: &[&str], dir: &Path) -> Output {
    // Print the command being executed
    let cmd_str = format!("> {} {}", cmd, args.join(" "));
    println!("{}", cmd_str);
    
    // Run the command
    let output = run_command(cmd, args, dir);
    
    // Print the output
    println!("{}", String::from_utf8_lossy(&output.stdout));
    
    output
}

fn run_vnext(dir: &Path) -> String {
    // Build the binary first in the project directory
    let project_dir = std::env::current_dir().expect("Failed to get current directory");
    println!("> Building vnext binary");
    Command::new("cargo")
        .args(["build"])
        .current_dir(&project_dir)
        .output()
        .expect("Failed to build vnext");
    
    // Get the path to the built binary
    let binary_path = project_dir.join("target/debug/vnext");
    println!("> Running {} in {:?}", binary_path.display(), dir);
    
    // Run the binary in the specified directory
    let output = Command::new(binary_path)
        .current_dir(&dir)
        .output()
        .expect("Failed to execute vnext");
    
    let version = String::from_utf8_lossy(&output.stdout).trim().to_string();

    println!("Version: {}", version);

    version
}

#[test]
fn integration_tests() {

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
        "### What's changed in v2.1.1\n\n* non-conventional: bump"
    );
    assert_eq!(
        changelog, expected_changelog,
        "Changelog output should match expected format for version 2.1.1"
    );

    let tag_name = format!("v{}", version);
    run_and_show_command("git", &["tag", &tag_name], repo_path);
}

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
        "### What's changed in v0.1.1\n\n* fix: 1\n* fix: 2"
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
        "### What's changed in v1.0.0\n\n* feat: add new feature\n\nBREAKING CHANGE: This removes the old API"
    );

    assert_eq!(
        changelog, expected_changelog,
        "Changelog output should match expected format"
    );
    
}

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
    assert!(changelog.contains("fix: first bugfix"), "Changelog should contain new feature");
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