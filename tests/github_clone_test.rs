use std::path::Path;
use std::process::{Command, Output};
use tempfile::tempdir;

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

#[test]
fn test_github_integration_with_real_repo() {
    // Create a temporary directory for the clone
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let repo_path = temp_dir.path();
    
    // Clone the vnext repository
    let repo_url = "https://github.com/unbounded-tech/vnext.git";
    println!("Cloning repository {} to {}", repo_url, repo_path.display());
    
    let output = run_and_show_command("git", &["clone", repo_url, repo_path.to_str().unwrap()], Path::new("."));
    assert!(output.status.success(), "Failed to clone repository");
    
    // Run vnext with --github and --changelog flags
    println!("Running vnext with --github and --changelog flags");
    let project_dir = std::env::current_dir().expect("Failed to get current directory");
    let binary_path = project_dir.join("target/debug/vnext");
    
    // Build the binary first
    Command::new("cargo")
        .args(["build"])
        .output()
        .expect("Failed to build vnext");
    
    let output = Command::new(&binary_path)
        .args(["--github", "--changelog"])
        .current_dir(&repo_path)
        .output()
        .expect("Failed to execute vnext with --github and --changelog");
    
    let changelog = String::from_utf8_lossy(&output.stdout).to_string();
    println!("Changelog output:\n{}", changelog);
    
    // Verify that the latest commit in the changelog has an author attribution
    assert!(changelog.contains("(by @"), 
        "Changelog should contain author attribution in format '(by @username)'");
}