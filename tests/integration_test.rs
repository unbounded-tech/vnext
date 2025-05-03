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

    // 1. Run against current directory as sanity check
    println!("Running vnext in current directory");
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    println!("Current directory: {:?}", current_dir);
    let version = run_vnext(&current_dir);

    // version should be greater than 0.0.0, but it's a string so we need to parse it
    let version_parts: Vec<&str> = version.split('.').collect();
    assert!(version_parts.len() == 3, "Version should be in the format x.y.z");
    let major: u32 = version_parts[0].parse().expect("Failed to parse major version");
    let minor: u32 = version_parts[1].parse().expect("Failed to parse minor version");
    let patch: u32 = version_parts[2].parse().expect("Failed to parse patch version");
    assert!(major > 0 || minor > 0 || patch > 0, "Version should be greater than 0.0.0");
    println!("Asserted version {} is greater than 0.0.0", version);

    // 2. Run vnext on empty directory
    print!("Running vnext in empty directory");
    let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");
    let repo_path = temp_dir.path();
    println!("Temporary directory created at: {:?}", repo_path);
    let version = run_vnext(repo_path);
    assert_eq!(version, "0.0.0", "Version should be 0.0.0 on empty repo");
    println!("Asserted version {} is 0.0.0", version);

    // 3. Initialize the directory as a git repository  
    println!("Initializing git repository in temporary directory, and running vnext again");  
    run_and_show_command("git", &["init"], repo_path);
    run_and_show_command("git", &["config", "user.name", "patrickleet"], repo_path);
    run_and_show_command("git", &["config", "user.email", "pat@patscott.io"], repo_path);

    let version = run_vnext(repo_path);
    assert_eq!(version, "0.0.0", "Version should still be 0.0.0 after git init");
    println!("Asserted version {} is still 0.0.0", version);
    
    // 4. Create a README file and commit it
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

