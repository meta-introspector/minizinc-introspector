use clap::{Parser, Subcommand};
use std::path::PathBuf; // Add this import

pub mod build;
pub mod test;
pub mod run;
pub mod debug;
pub mod clean;
pub mod extract_constants;
pub mod generate_minizinc_params;
pub mod generate_constants_file;
pub mod ast_to_minizinc;
pub mod code_search;
pub mod self_optimize;
pub mod test_ast_to_minizinc;
pub mod analyze_duplicates;
pub mod index_update;
pub mod command_handler;
pub mod utils;
pub mod constants;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, disable_help_flag = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Commands {
    /// Builds project components
    Build(build::BuildArgs),
    /// Runs project tests
    Test(test::TestArgs),
    /// Executes MiniZinc models
    Run(run::RunArgs),
    /// Provides debugging utilities
    Debug(debug::DebugArgs),
    /// Cleans build artifacts
    Clean(clean::CleanArgs),
    /// Extracts constant strings from the codebase using MiniZinc
    ExtractConstants(extract_constants::ExtractConstantsArgs),
    /// Generates MiniZinc parameters from extracted constants
    GenerateParams(generate_minizinc_params::GenerateParamsArgs),
    /// Generates constants.rs file based on MiniZinc proof
    GenerateConstantsFile(generate_constants_file::GenerateConstantsFileArgs),
    /// Analyzes constant usage in the codebase
    AnalyzeConstants,
    /// Converts Rust AST to MiniZinc model and data
    AstToMiniZinc(ast_to_minizinc::AstToMiniZincArgs),
    /// Performs systematic code search
    CodeSearch(code_search::CodeSearchArgs),
    /// Bootstraps the entire ZOS system
    Bootstrap {
        /// The specific bootstrap target (e.g., "zos")
        target: String,
    },
    /// Self-optimizes the codebase using MiniZinc and LLM
    SelfOptimize(self_optimize::SelfOptimizeArgs),
    /// Tests AST to MiniZinc conversion for a single file
    TestAstToMiniZinc(test_ast_to_minizinc::TestAstToMiniZincArgs),
    /// Finds duplicate or similar code within the codebase
    AnalyzeDuplicates(AnalyzeDuplicatesArgs),
    /// Updates the project index incrementally with status reports
    IndexUpdate(IndexUpdateArgs),
}

#[derive(Args, Clone, Debug)]
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

#[derive(Args, Clone, Debug)]
pub struct IndexUpdateArgs {
    /// Perform an incremental update of the index
    #[arg(long, default_value_t = true)]
    pub incremental: bool,
}
