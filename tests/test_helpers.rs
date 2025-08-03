use std::path::Path;
use std::process::{Command, Output};

// Helper function to run a command and return its output
pub fn run_command(cmd: &str, args: &[&str], dir: &Path) -> Output {
    Command::new(cmd)
        .args(args)
        .current_dir(dir)
        .output()
        .expect(&format!("Failed to execute command: {} {:?}", cmd, args))
}

// Helper function to run a command, print it, and show its output
pub fn run_and_show_command(cmd: &str, args: &[&str], dir: &Path) -> Output {
    // Print the command being executed
    let cmd_str = format!("> {} {}", cmd, args.join(" "));
    println!("{}", cmd_str);
    
    // Run the command
    let output = run_command(cmd, args, dir);
    
    // Print the output
    println!("{}", String::from_utf8_lossy(&output.stdout));
    
    output
}

pub fn run_vnext(dir: &Path) -> String {
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