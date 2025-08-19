use clap::{Parser, Subcommand};
use std::path::PathBuf; // Add this import
use crate::commands;

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
    /// Tests AST to MiniZinc conversion for a single file
    TestAstToMiniZinc(commands::test_ast_to_minizinc::TestAstToMiniZincArgs),
    /// Finds duplicate or similar code within the codebase
    AnalyzeDuplicates(AnalyzeDuplicatesArgs), // New subcommand
}

#[derive(Parser)]
pub struct AnalyzeDuplicatesArgs {
    /// The suggested code to analyze for duplicates (can be a string or a file path)
    #[arg(long)]
    pub suggested_code: String,
    /// The root directory to search for duplicate code
    #[arg(long)]
    pub search_path: PathBuf,
    /// If true, treat suggested_code as a file path, otherwise as a direct string
    #[arg(long, default_value_t = false)]
    pub is_file: bool,
}
