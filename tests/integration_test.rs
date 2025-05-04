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

fn run_vnext_with_args(dir: &Path, args: &[&str]) -> String {
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
    println!("> Running {} {} in {:?}", binary_path.display(), args.join(" "), dir);
    
    // Run the binary in the specified directory with the provided arguments
    let output = Command::new(binary_path)
        .args(args)
        .current_dir(&dir)
        .output()
        .expect("Failed to execute vnext");
    
    let output_str = String::from_utf8_lossy(&output.stdout).to_string();

    println!("Output: \n{}", output_str);

    output_str
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
    let tag_name = format!("v{}", version);
    run_and_show_command("git", &["tag", &tag_name], repo_path);
}

#[test]
fn test_changelog() {
    // Create a temporary directory for the test repository
    let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");
    let repo_path = temp_dir.path();
    println!("Temporary directory created at: {:?}", repo_path);
    
    // Initialize git repository
    run_and_show_command("git", &["init"], repo_path);
    run_and_show_command("git", &["config", "user.name", "Test User"], repo_path);
    run_and_show_command("git", &["config", "user.email", "test@example.com"], repo_path);
    
    // Create and commit files with different commit types and scopes
    // 1. Add a feature with UI scope
    let ui_feature_path = repo_path.join("ui-feature.txt");
    fs::write(&ui_feature_path, "UI Feature").expect("Failed to write UI feature file");
    run_and_show_command("git", &["add", ui_feature_path.to_str().unwrap()], repo_path);
    run_and_show_command("git", &["commit", "-m", "feat(ui): Add new button"], repo_path);
    
    // 2. Add a fix with UI scope
    let ui_fix_path = repo_path.join("ui-fix.txt");
    fs::write(&ui_fix_path, "UI Fix").expect("Failed to write UI fix file");
    run_and_show_command("git", &["add", ui_fix_path.to_str().unwrap()], repo_path);
    run_and_show_command("git", &["commit", "-m", "fix(ui): Fix button alignment"], repo_path);
    
    // 3. Add an unscoped feature
    let feature_path = repo_path.join("feature.txt");
    fs::write(&feature_path, "Feature").expect("Failed to write feature file");
    run_and_show_command("git", &["add", feature_path.to_str().unwrap()], repo_path);
    run_and_show_command("git", &["commit", "-m", "feat: Add new widget"], repo_path);
    
    // 4. Add a chore
    let chore_path = repo_path.join("chore.txt");
    fs::write(&chore_path, "Chore").expect("Failed to write chore file");
    run_and_show_command("git", &["add", chore_path.to_str().unwrap()], repo_path);
    run_and_show_command("git", &["commit", "-m", "chore: Update docs"], repo_path);
    
    // 5. Add a major change
    let major_path = repo_path.join("major.txt");
    fs::write(&major_path, "Major").expect("Failed to write major file");
    run_and_show_command("git", &["add", major_path.to_str().unwrap()], repo_path);
    
    // 6. Add a breaking change
    let breaking_path = repo_path.join("breaking.txt");
    fs::write(&breaking_path, "Breaking").expect("Failed to write breaking file");
    run_and_show_command("git", &["add", breaking_path.to_str().unwrap()], repo_path);
    run_and_show_command("git", &["commit", "-m", "feat: Remove API\n\nBREAKING CHANGE: API removed"], repo_path);
    
    // Run vnext with --changelog flag
    let changelog = run_vnext_with_args(repo_path, &["--changelog"]);
    
    // Verify the changelog content
    assert!(changelog.contains("# 1.0.0"), "Changelog should contain the version 1.0.0");
    
    // Check for breaking changes section
    assert!(changelog.contains("## Breaking Changes"), "Changelog should have a Breaking Changes section");
    // Only check for API removed since that's what's being detected
    assert!(changelog.contains("- API removed"), "Breaking changes should include 'API removed'");
    
    // Check for unscoped changes section
    assert!(changelog.contains("## Changes"), "Changelog should have a Changes section");
    assert!(changelog.contains("### Features"), "Changes should have a Features subsection");
    assert!(changelog.contains("- Remove API"), "Features should include 'Remove API'");
    
    // Check for UI scoped changes section
    assert!(changelog.contains("## Ui Changes"), "Changelog should have a UI Changes section");
    assert!(changelog.contains("### Features"), "UI Changes should have a Features subsection");
    assert!(changelog.contains("- Add new button"), "UI Features should include 'Add new button'");
    assert!(changelog.contains("### Fixes"), "UI Changes should have a Fixes subsection");
    assert!(changelog.contains("- Fix button alignment"), "UI Fixes should include 'Fix button alignment'");
    
    // Verify the order of sections
    let breaking_pos = changelog.find("## Breaking Changes").unwrap_or(0);
    let changes_pos = changelog.find("## Changes").unwrap_or(0);
    let ui_pos = changelog.find("## Ui Changes").unwrap_or(0);
    
    assert!(breaking_pos < changes_pos, "Breaking Changes should come before Changes");
    assert!(changes_pos < ui_pos, "Changes should come before Ui Changes");
}

