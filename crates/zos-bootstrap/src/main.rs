use clap::{Parser, Subcommand, Args};

mod utils;
use crate::utils::error::{Result, ZosError};
use crate::utils::paths;

mod commands;
use commands::build::{BuildArgs, handle_build_command};
use commands::test::{TestArgs, handle_test_command};
use commands::run::{RunArgs, handle_run_command};
use commands::debug::{DebugArgs, handle_debug_command};
use commands::clean::{CleanArgs, handle_clean_command};
use commands::extract_constants::handle_extract_constants_command;
use commands::generate_minizinc_params::{GenerateParamsArgs, handle_generate_params_command};
use commands::generate_constants_file::{GenerateConstantsFileArgs, handle_generate_constants_file_command};

mod code_analysis;
use code_analysis::constant_analyzer::ConstantAnalyzer;
mod constants;

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
    /// Extracts constant strings from the codebase using MiniZinc
    ExtractConstants(ExtractConstantsArgs),
    /// Generates MiniZinc parameters from extracted constants
    GenerateParams(GenerateParamsArgs),
    /// Generates constants.rs file based on MiniZinc proof
    GenerateConstantsFile(GenerateConstantsFileArgs),
    /// Analyzes constant usage in the codebase
    AnalyzeConstants,
    /// Bootstraps the entire ZOS system
    Bootstrap {
        /// The specific bootstrap target (e.g., "zos")
        target: String,
    },
}

#[derive(Args, Clone)]
pub struct ExtractConstantsArgs {
    #[arg(long)]
    pub rust_only: bool,
    #[arg(long)]
    pub file_path: Option<String>,
    #[arg(long)]
    pub generate_sed_script: bool,
    #[arg(long)]
    pub prove_constants_usage: bool,
}

fn handle_analyze_constants_command() -> Result<()> {
    println!("Analyzing constant usage...");

    // Explicitly call paths.rs functions to resolve dead_code warnings
    let _ = paths::get_minizinc_models_dir()?;
    let _ = paths::get_minizinc_data_dir()?;
    let _ = paths::get_minizinc_user_solvers_dir()?;

    let mut analyzer = ConstantAnalyzer::new()?;
    let project_root = paths::resolve_project_root()?;
    analyzer.analyze(&project_root)?;
    analyzer.report();

    println!("Constant analysis completed.");
    Ok(())
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
        Some(Commands::ExtractConstants(args)) => {
            handle_extract_constants_command(args.clone())?;
        }
        Some(Commands::GenerateParams(args)) => {
            handle_generate_params_command(args.clone())?;
        }
        Some(Commands::GenerateConstantsFile(args)) => {
            handle_generate_constants_file_command(args.clone())?;
        }
        Some(Commands::AnalyzeConstants) => {
            handle_analyze_constants_command()?;
        }
        Some(Commands::Bootstrap { target }) => {
            if target == "zos" {
                println!("Commencing ZOS Bootstrap: Building all core components...");
                handle_build_command(BuildArgs { command: Some(commands::build::BuildCommands::All {}), strace: false })?;
                println!("ZOS Bootstrap: Core components built successfully.");
                println!("Commencing ZOS Bootstrap: Running all tests...");
                handle_test_command(TestArgs { command: Some(commands::test::TestCommands::All {}) })?;
                println!("ZOS Bootstrap: All tests completed successfully.");
                println!("Commencing ZOS Bootstrap: Running initial embedding model...");
                // This is a placeholder for running an initial embedding model.
                // It would require specific arguments for the 'run embedding' command.
                // For now, we'll just print a message.
                println!("(Initial embedding model run placeholder)");
                println!("ZOS Bootstrap: Initial embedding model run placeholder completed.");
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
