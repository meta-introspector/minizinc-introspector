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
}
