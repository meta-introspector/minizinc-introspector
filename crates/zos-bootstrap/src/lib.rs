pub mod utils;
pub mod commands;
pub mod code_analysis;
pub mod constants;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Builds project components
    Build(commands::build::BuildArgs),
    /// Runs project tests
    Test(commands::test::TestArgs),
    /// Executes MiniZinc models
    Run(commands::run::RunArgs),
    /// Provides debugging utilities
    Debug(commands::debug::DebugArgs),
    /// Cleans build artifacts
    Clean(commands::clean::CleanArgs),
    /// Extracts constant strings from the codebase using MiniZinc
    ExtractConstants(commands::extract_constants::ExtractConstantsArgs),
    /// Generates MiniZinc parameters from extracted constants
    GenerateParams(commands::generate_minizinc_params::GenerateParamsArgs),
    /// Generates constants.rs file based on MiniZinc proof
    GenerateConstantsFile(commands::generate_constants_file::GenerateConstantsFileArgs),
    /// Analyzes constant usage in the codebase
    AnalyzeConstants,
    /// Converts Rust AST to MiniZinc model and data
    AstToMiniZinc(commands::ast_to_minizinc::AstToMiniZincArgs),
    /// Performs systematic code search
    CodeSearch(commands::code_search::CodeSearchArgs),
    /// Bootstraps the entire ZOS system
    Bootstrap {
        /// The specific bootstrap target (e.g., "zos")
        target: String,
    },
    /// Self-optimizes the codebase using MiniZinc and LLM
    SelfOptimize(commands::self_optimize::SelfOptimizeArgs),
}

// Add a main function to handle the commands, as this is a library crate
// This main function will be called by the actual binary (e.g., in ragit-commands)
pub fn main() -> Result<(), String> {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Build(args)) => commands::build::handle_build_command(args).map_err(|e| format!("Build command failed: {}", e)),
        Some(Commands::Test(args)) => commands::test::handle_test_command(args).map_err(|e| format!("Test command failed: {}", e)),
        Some(Commands::Run(args)) => commands::run::handle_run_command(args).map_err(|e| format!("Run command failed: {}", e)),
        Some(Commands::Debug(args)) => commands::debug::handle_debug_command(args).map_err(|e| format!("Debug command failed: {}", e)),
        Some(Commands::Clean(args)) => commands::clean::handle_clean_command(args).map_err(|e| format!("Clean command failed: {}", e)),
        Some(Commands::ExtractConstants(args)) => commands::extract_constants::handle_extract_constants_command(args).map_err(|e| format!("ExtractConstants command failed: {}", e)),
        Some(Commands::GenerateParams(args)) => commands::generate_minizinc_params::handle_generate_params_command(args).map_err(|e| format!("GenerateParams command failed: {}", e)),
        Some(Commands::GenerateConstantsFile(args)) => commands::generate_constants_file::handle_generate_constants_file_command(args).map_err(|e| format!("GenerateConstantsFile command failed: {}", e)),
        Some(Commands::AnalyzeConstants) => {
            // Placeholder for AnalyzeConstants logic
            println!("AnalyzeConstants command executed.");
            Ok(())
        },
        Some(Commands::AstToMiniZinc(args)) => commands::ast_to_minizinc::handle_ast_to_minizinc_command(args).map_err(|e| format!("AstToMiniZinc command failed: {}", e)),
        Some(Commands::CodeSearch(args)) => commands::code_search::handle_code_search_command(args).map_err(|e| format!("CodeSearch command failed: {}", e)),
        Some(Commands::Bootstrap { target }) => {
            // Placeholder for Bootstrap logic
            println!("Bootstrap command executed for target: {}", target);
            Ok(())
        },
        Some(Commands::SelfOptimize(args)) => commands::self_optimize::handle_self_optimize_command(args).map_err(|e| format!("SelfOptimize command failed: {}", e)),
        None => {
            println!("No command provided. Use --help for more information.");
            Ok(())
        }
    }
}
