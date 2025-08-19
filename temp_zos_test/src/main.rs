use zos_bootstrap::cli::{Cli, Commands};
use clap::Parser; // Import Parser trait

fn main() -> Result<(), String> {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Build(args)) => zos_bootstrap::commands::build::handle_build_command(args).map_err(|e| format!("Build command failed: {}", e)),
        Some(Commands::Test(args)) => zos_bootstrap::commands::test::handle_test_command(args).map_err(|e| format!("Test command failed: {}", e)),
        Some(Commands::Run(args)) => zos_bootstrap::commands::run::handle_run_command(args).map_err(|e| format!("Run command failed: {}", e)),
        Some(Commands::Debug(args)) => zos_bootstrap::commands::debug::handle_debug_command(args).map_err(|e| format!("Debug command failed: {}", e)),
        Some(Commands::Clean(args)) => zos_bootstrap::commands::clean::handle_clean_command(args).map_err(|e| format!("Clean command failed: {}", e)),
        Some(Commands::ExtractConstants(args)) => zos_bootstrap::commands::extract_constants::handle_extract_constants_command(args).map_err(|e| format!("ExtractConstants command failed: {}", e)),
        Some(Commands::GenerateParams(args)) => zos_bootstrap::commands::generate_minizinc_params::handle_generate_params_command(args).map_err(|e| format!("GenerateParams command failed: {}", e)),
        Some(Commands::GenerateConstantsFile(args)) => zos_bootstrap::commands::generate_constants_file::handle_generate_constants_file_command(args).map_err(|e| format!("GenerateConstantsFile command failed: {}", e)),
        Some(Commands::AnalyzeConstants) => {
            // Placeholder for AnalyzeConstants logic
            println!("AnalyzeConstants command executed.");
            Ok(())
        },
        Some(Commands::AstToMiniZinc(args)) => zos_bootstrap::commands::ast_to_minizinc::handle_ast_to_minizinc_command(args).map_err(|e| format!("AstToMiniZinc command failed: {}", e)),
        Some(Commands::CodeSearch(args)) => zos_bootstrap::commands::code_search::handle_code_search_command(args).map_err(|e| format!("CodeSearch command failed: {}", e)),
        Some(Commands::Bootstrap { target }) => {
            // Placeholder for Bootstrap logic
            println!("Bootstrap command executed for target: {}", target);
            Ok(())
        },
        Some(Commands::SelfOptimize(args)) => zos_bootstrap::commands::self_optimize::handle_self_optimize_command(args).map_err(|e| format!("SelfOptimize command failed: {}", e)),
        None => {
            println!("No command provided. Use --help for more information.");
            Ok(())
        }
    }
}
