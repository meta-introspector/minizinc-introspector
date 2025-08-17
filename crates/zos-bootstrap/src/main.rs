use clap::{Parser, Subcommand};

mod utils;
use crate::utils::error::{Result, ZosError};
use crate::utils::paths;
use crate::utils::subprocess;

mod commands;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Builds project components
    Build {},
    /// Runs project tests
    Test {},
    /// Executes MiniZinc models
    Run {},
    /// Provides debugging utilities
    Debug {},
    /// Cleans build artifacts
    Clean {},
    /// Bootstraps the entire ZOS system
    Bootstrap {
        /// The specific bootstrap target (e.g., "zos")
        target: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Build {}) => {
            println!("Build command received");
            // TODO: Implement build logic
        }
        Some(Commands::Test {}) => {
            println!("Test command received");
            // TODO: Implement test logic
        }
        Some(Commands::Run {}) => {
            println!("Run command received");
            // TODO: Implement run logic
        }
        Some(Commands::Debug {}) => {
            println!("Debug command received");
            // TODO: Implement debug logic
        }
        Some(Commands::Clean {}) => {
            println!("Clean command received");
            // TODO: Implement clean logic
        }
        Some(Commands::Bootstrap { target }) => {
            if target == "zos" {
                println!("Commencing ZOS Bootstrap: Building all core components...");
                // TODO: Call build all logic
                println!("Commencing ZOS Bootstrap: Running all tests...");
                // TODO: Call test all logic
                println!("Commencing ZOS Bootstrap: Running initial embedding model...");
                // TODO: Call run initial embedding model logic
                println!("ZOS Bootstrap Complete.");
            } else {
                return Err(ZosError::InvalidArgument(format!("Unknown bootstrap target: {}. Only 'zos' is supported currently.", target)));
            }
        }
        None => {
            println!("No command provided. Use --help for more information.");
        }
    }
    Ok(())
}
