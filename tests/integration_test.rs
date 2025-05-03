use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

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

// Helper function to initialize a git repository
fn init_git_repo(dir: &Path) {
    run_command("git", &["init"], dir);
    run_command("git", &["config", "user.name", "Test User"], dir);
    run_command("git", &["config", "user.email", "test@example.com"], dir);
}

// Helper function to run vnext
fn run_vnext(dir: &Path) -> String {
    let output = Command::new("cargo")
        .args(["run"])
        .current_dir(&dir)
        .output()
        .expect("Failed to execute vnext");
    
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

#[test]
fn test_basic_workflow() {
    // Create a temporary directory for the repository
    let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");
    let repo_path = temp_dir.path();

    println!("Temporary directory created at: {:?}", repo_path);
    
    // Initialize git repository
    init_git_repo(repo_path);
    println!("Initialized git repository at: {:?}", repo_path);
    
    // Create a README file
    let readme_path = repo_path.join("README.md");
    fs::write(&readme_path, "# Test Repository").expect("Failed to write README file");

    println!("Created README file at: {:?}", readme_path);
    // Show git status and perform git operations
    run_and_show_command("git", &["status"], repo_path);
    run_and_show_command("git", &["add", readme_path.to_str().unwrap()], repo_path);
    run_and_show_command("git", &["commit", "-m", "feat: Initial commit"], repo_path);
    let version = run_and_show_command("vnext", &[], repo_path);

    // assert version's output is "0.1.0"
    assert_eq!(String::from_utf8_lossy(&version.stdout), "0.1.0\n", "Initial version should be 0.1.0");

    
    // // Add tag for the initial version
    // add_tag(repo_path, &format!("v{}", version));
    
    // // Add a patch commit
    // add_commit(repo_path, "fix: bug fix", true);
    
    // // Run vnext - should return 0.1.1
    // let version = run_vnext(repo_path);
    // assert_eq!(version, "0.1.1", "Patch commit should bump patch version");
    
    // // Add tag for the patch version
    // add_tag(repo_path, &format!("v{}", version));
    
    // // Add a minor commit
    // add_commit(repo_path, "feat: new feature", true);
    
    // // Run vnext - should return 0.2.0
    // let version = run_vnext(repo_path);
    // assert_eq!(version, "0.2.0", "Feature commit should bump minor version");
    
    // // Add tag for the minor version
    // add_tag(repo_path, &format!("v{}", version));
    
    // // Add a major commit
    // add_commit(repo_path, "BREAKING CHANGE: major update", true);
    
    // // Run vnext - should return 1.0.0
    // let version = run_vnext(repo_path);
    // assert_eq!(version, "1.0.0", "Breaking change should bump major version");
}

