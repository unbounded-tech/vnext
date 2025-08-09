use std::fs;
use std::process::Command;

// Import the test_helpers module
mod test_helpers;
use test_helpers::{run_and_show_command, run_vnext};

// Helper function to run vnext with the --current flag
fn run_vnext_current(dir: &std::path::Path) -> String {
    // Get the path to the built binary
    let project_dir = std::env::current_dir().expect("Failed to get current directory");
    let binary_path = project_dir.join("target/debug/vnext");
    println!("> Running {} --current in {:?}", binary_path.display(), dir);
    
    // Run the binary with the --current flag in the specified directory
    let output = Command::new(binary_path)
        .args(["--current"])
        .current_dir(&dir)
        .output()
        .expect("Failed to execute vnext --current");
    
    let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
    println!("Current Version: {}", version);
    
    version
}

#[test]
fn current_flag_tests() {
    // 1. Run vnext --current on empty directory
    print!("Running vnext --current in empty directory");
    let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");
    let repo_path = temp_dir.path();
    println!("Temporary directory created at: {:?}", repo_path);
    let version = run_vnext_current(repo_path);
    assert_eq!(version, "0.0.0", "Current version should be 0.0.0 on empty repo");
    println!("Asserted current version {} is 0.0.0", version);

    // 2. Initialize the directory as a git repository  
    println!("Initializing git repository in temporary directory, and running vnext --current again");  
    run_and_show_command("git", &["init"], repo_path);
    run_and_show_command("git", &["config", "user.name", "patrickleet"], repo_path);
    run_and_show_command("git", &["config", "user.email", "pat@patscott.io"], repo_path);

    let version = run_vnext_current(repo_path);
    assert_eq!(version, "0.0.0", "Current version should still be 0.0.0 after git init");
    println!("Asserted current version {} is still 0.0.0", version);
    
    // 3. Create a README file and commit it
    println!("Creating README file and committing it, then running vnext --current again");
    let readme_path = repo_path.join("README.md");
    fs::write(&readme_path, "# Test Repository").expect("Failed to write README file");
    println!("Created README file at: {:?}", readme_path);

    run_and_show_command("git", &["status"], repo_path);
    run_and_show_command("git", &["add", readme_path.to_str().unwrap()], repo_path);
    run_and_show_command("git", &["commit", "-m", "feat: Initial commit"], repo_path);
    
    // Current version should still be 0.0.0 since no tag has been created
    let version = run_vnext_current(repo_path);
    assert_eq!(version, "0.0.0", "Current version should still be 0.0.0 before tagging");
    println!("Asserted current version {} is still 0.0.0", version);

    // Run vnext to get the next version and tag it
    let next_version = run_vnext(repo_path);
    assert_eq!(next_version, "0.1.0", "Next version should be 0.1.0");
    
    // Add tag for the initial version
    let tag_name = format!("v{}", next_version);
    run_and_show_command("git", &["tag", &tag_name], repo_path);
    
    // Current version should now be 0.1.0 after tagging
    let version = run_vnext_current(repo_path);
    assert_eq!(version, "0.1.0", "Current version should be 0.1.0 after tagging");
    println!("Asserted current version {} is 0.1.0", version);

    // 4. Add a file "patch", and a patch commit
    println!("Creating patch file and committing it, then running vnext --current again");
    let patch_path = repo_path.join("patch");
    fs::write(&patch_path, "This is a patch").expect("Failed to write patch file");
    println!("Created patch file at: {:?}", patch_path);
    run_and_show_command("git", &["status"], repo_path);
    run_and_show_command("git", &["add", patch_path.to_str().unwrap()], repo_path);
    run_and_show_command("git", &["commit", "-m", "fix: Bug fix"], repo_path);

    // Current version should still be 0.1.0 since we haven't tagged the new version
    let version = run_vnext_current(repo_path);
    assert_eq!(version, "0.1.0", "Current version should still be 0.1.0 before tagging the patch");
    println!("Asserted current version {} is still 0.1.0", version);
    
    // Run vnext to get the next version and tag it
    let next_version = run_vnext(repo_path);
    assert_eq!(next_version, "0.1.1", "Next version should be 0.1.1");
    
    let tag_name = format!("v{}", next_version);
    run_and_show_command("git", &["tag", &tag_name], repo_path);
    
    // Current version should now be 0.1.1 after tagging
    let version = run_vnext_current(repo_path);
    assert_eq!(version, "0.1.1", "Current version should be 0.1.1 after tagging the patch");
    println!("Asserted current version {} is 0.1.1", version);
}