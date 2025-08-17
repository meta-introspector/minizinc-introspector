use clap::{Parser, Subcommand};

mod utils;
use crate::utils::error::{Result, ZosError};

mod commands;
use commands::build::{BuildArgs, handle_build_command};
use commands::test::{TestArgs, handle_test_command};
use commands::run::{RunArgs, handle_run_command};
use commands::debug::{DebugArgs, handle_debug_command};
use commands::clean::{CleanArgs, handle_clean_command};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Builds project components
    Build(BuildArgs),
    /// Runs project tests
    Test(TestArgs),
    /// Executes MiniZinc models
    Run(RunArgs),
    /// Provides debugging utilities
    Debug(DebugArgs),
    /// Cleans build artifacts
    Clean(CleanArgs),
    /// Bootstraps the entire ZOS system
    Bootstrap {
        /// The specific bootstrap target (e.g., "zos")
        target: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Build(args)) => {
            handle_build_command(args.clone())?;
        }
        Some(Commands::Test(args)) => {
            handle_test_command(args.clone())?;
        }
        Some(Commands::Run(args)) => {
            handle_run_command(args.clone())?;
        }
        Some(Commands::Debug(args)) => {
            handle_debug_command(args.clone())?;
        }
        Some(Commands::Clean(args)) => {
            handle_clean_command(args.clone())?;
        }
        Some(Commands::Bootstrap { target }) => {
            if target == "zos" {
                println!("Commencing ZOS Bootstrap: Building all core components...");
                handle_build_command(BuildArgs { command: Some(commands::build::BuildCommands::All {}), strace: false })?;
                println!("Commencing ZOS Bootstrap: Running all tests...");
                handle_test_command(TestArgs { command: Some(commands::test::TestCommands::All {}) })?;
                println!("Commencing ZOS Bootstrap: Running initial embedding model...");
                // This is a placeholder for running an initial embedding model.
                // It would require specific arguments for the 'run embedding' command.
                // For now, we'll just print a message.
                println!("(Initial embedding model run placeholder)");
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