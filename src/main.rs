//! Entry point for the vnext application

use clap::Parser;
use log::debug;

use vnext::cli::{Cli, run};
use vnext::utils::logging;

/// Main function
fn main() {
    // Initialize logging
    logging::init_logging().expect("Failed to setup logging");
    debug!("Starting vnext");

    // Parse command line arguments
    let cli = Cli::parse();

    // Run the CLI
    if let Err(e) = run(cli) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    debug!("vnext completed successfully");
}
