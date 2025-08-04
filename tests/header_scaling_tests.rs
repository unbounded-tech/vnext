use std::fs;
use std::process::Command;

// Import the test_helpers module
mod test_helpers;
use test_helpers::{run_and_show_command, run_vnext};

#[test]
fn test_header_scaling() {
    // Create a new temp dir
    let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");
    let repo_path = temp_dir.path();
    println!("Temporary directory created at: {:?}", repo_path);

    // Initialize as git repo
    run_and_show_command("git", &["init"], repo_path);
    run_and_show_command("git", &["config", "user.name", "patrickleet"], repo_path);
    run_and_show_command("git", &["config", "user.email", "pat@patscott.io"], repo_path);

    // Add a commit with markdown headers in the body to test header scaling
    let file_path = repo_path.join("header-scaling.md");
    fs::write(&file_path, "# Header Scaling Test").expect("Failed to write file");
    run_and_show_command("git", &["add", file_path.to_str().unwrap()], repo_path);
    run_and_show_command(
        "git",
        &["commit", "-m", "feat: add header scaling test\n\n# H1 Header Should Scale to H4\n\n## H2 Header Should Scale to H5\n\n### H3 Header Should Scale to H6\n\n#### H4 Header Should Also Scale to H6\n\n##### H5 Header Should Also Scale to H6\n\nRegular text should remain unchanged"],
        repo_path
    );

    // Run vnext and check version
    let version = run_vnext(repo_path);
    assert_eq!(version, "0.1.0", "First run version should be 0.1.0");
    println!("Asserted version {} is 0.1.0", version);

    // Test 1: Check changelog with header scaling enabled (default)
    println!("Running vnext with --changelog to verify header scaling (enabled)");
    let project_dir = std::env::current_dir().expect("Failed to get current directory");
    let binary_path = project_dir.join("target/debug/vnext");
    let output = Command::new(&binary_path)
        .args(["--changelog"])
        .current_dir(&repo_path)
        .output()
        .expect("Failed to execute vnext with --changelog");
    
    let changelog = String::from_utf8_lossy(&output.stdout).to_string();
    println!("Changelog output with header scaling:\n{}", changelog);
    
    // Verify changelog formatting
    let changelog = changelog.trim_end().to_string();
    
    // Check that the version header is present
    assert!(changelog.contains("### What's changed in v0.1.0"), "Changelog should mention version 0.1.0");
    
    // Check that commit title is present
    assert!(changelog.contains("* feat: add header scaling test"), "Changelog should contain commit title");
    
    // Check that header scaling works correctly
    assert!(changelog.contains("  #### H1 Header Should Scale to H4"), "H1 should be scaled to H4");
    assert!(changelog.contains("  ##### H2 Header Should Scale to H5"), "H2 should be scaled to H5");
    assert!(changelog.contains("  ###### H3 Header Should Scale to H6"), "H3 should be scaled to H6");
    assert!(changelog.contains("  Regular text should remain unchanged"), "Regular text should remain unchanged");
    
    // Test 2: Check changelog with header scaling disabled
    println!("Running vnext with --changelog --no-header-scaling to verify disabled header scaling");
    let output_no_scaling = Command::new(&binary_path)
        .args(["--changelog", "--no-header-scaling"])
        .current_dir(&repo_path)
        .output()
        .expect("Failed to execute vnext with --changelog --no-header-scaling");
    
    let changelog_no_scaling = String::from_utf8_lossy(&output_no_scaling.stdout).to_string();
    println!("Changelog output without header scaling:\n{}", changelog_no_scaling);
    
    // Verify changelog formatting without header scaling
    let changelog_no_scaling = changelog_no_scaling.trim_end().to_string();
    
    // Check that the version header is present
    assert!(changelog_no_scaling.contains("### What's changed in v0.1.0"), "Changelog should mention version 0.1.0");
    
    // Check that commit title is present
    assert!(changelog_no_scaling.contains("* feat: add header scaling test"), "Changelog should contain commit title");
    
    // Check that headers are NOT scaled when disabled
    assert!(changelog_no_scaling.contains("  # H1 Header Should Scale to H4"), "H1 should not be scaled when disabled");
    assert!(changelog_no_scaling.contains("  ## H2 Header Should Scale to H5"), "H2 should not be scaled when disabled");
    assert!(changelog_no_scaling.contains("  ### H3 Header Should Scale to H6"), "H3 should not be scaled when disabled");
    assert!(changelog_no_scaling.contains("  #### H4 Header Should Also Scale to H6"), "H4 should not be scaled when disabled");
    assert!(changelog_no_scaling.contains("  ##### H5 Header Should Also Scale to H6"), "H5 should not be scaled when disabled");
    assert!(changelog_no_scaling.contains("  Regular text should remain unchanged"), "Regular text should remain unchanged");
}