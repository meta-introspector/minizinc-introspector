use crate::cli::{Cli, Commands};
use clap::Parser; // Import Parser trait
use crate::utils::error::{Result, ZosError}; // Assuming ZosError is in utils::error
use crate::commands;

pub fn handle_command_dispatch() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Build(args)) => commands::build::handle_build_command(args).map_err(|e| ZosError::Unknown(format!("Build command failed: {}", e))),
        Some(Commands::Test(args)) => commands::test::handle_test_command(args).map_err(|e| ZosError::Unknown(format!("Test command failed: {}", e))),
        Some(Commands::Run(args)) => commands::run::handle_run_command(args).map_err(|e| ZosError::Unknown(format!("Run command failed: {}", e))),
        Some(Commands::Debug(args)) => commands::debug::handle_debug_command(args).map_err(|e| ZosError::Unknown(format!("Debug command failed: {}", e))),
        Some(Commands::Clean(args)) => commands::clean::handle_clean_command(args).map_err(|e| ZosError::Unknown(format!("Clean command failed: {}", e))),
        Some(Commands::ExtractConstants(args)) => commands::extract_constants::handle_extract_constants_command(args).map_err(|e| ZosError::Unknown(format!("ExtractConstants command failed: {}", e))),
        Some(Commands::GenerateParams(args)) => commands::generate_minizinc_params::handle_generate_params_command(args).map_err(|e| ZosError::Unknown(format!("GenerateParams command failed: {}", e))),
        Some(Commands::GenerateConstantsFile(args)) => commands::generate_constants_file::handle_generate_constants_file_command(args).map_err(|e| ZosError::Unknown(format!("GenerateConstantsFile command failed: {}", e))),
        Some(Commands::AnalyzeConstants) => {
            // Placeholder for AnalyzeConstants logic
            println!("AnalyzeConstants command executed.");
            Ok(())
        },
        Some(Commands::AstToMiniZinc(args)) => commands::ast_to_minizinc::handle_ast_to_minizinc_command(args).map_err(|e| ZosError::Unknown(format!("AstToMiniZinc command failed: {}", e))),
        Some(Commands::CodeSearch(args)) => commands::code_search::handle_code_search_command(args).map_err(|e| ZosError::Unknown(format!("CodeSearch command failed: {}", e))),
        Some(Commands::Bootstrap { target }) => {
            // Placeholder for Bootstrap logic
            println!("Bootstrap command executed for target: {}", target);
            Ok(())
        },
        Some(Commands::SelfOptimize(args)) => commands::self_optimize::handle_self_optimize_command(args).map_err(|e| ZosError::Unknown(format!("SelfOptimize command failed: {}", e))),
        None => {
            println!("No command provided. Use --help for more information.");
            Ok(())
        }
    }
}
