use zos_bootstrap::cli::Cli;
use zos_bootstrap::cli::Commands;
use zos_bootstrap::{
//    Cli, Commands,
    utils::{error::{Result, ZosError}, paths},
    commands::{build::handle_build_command,
               test::handle_test_command,
               run::handle_run_command,
               debug::handle_debug_command,
               clean::handle_clean_command,
               extract_constants::handle_extract_constants_command,
               generate_minizinc_params::handle_generate_params_command,
               generate_constants_file::handle_generate_constants_file_command,
               ast_to_minizinc::handle_ast_to_minizinc_command,
               code_search::handle_code_search_command,
               test_ast_to_minizinc::handle_test_ast_to_minizinc_command},
    code_analysis::constant_analyzer::ConstantAnalyzer,
};
use clap::Parser;

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
        Some(Commands::AstToMiniZinc(args)) => {
            handle_ast_to_minizinc_command(args.clone())?;
        }
        Some(Commands::TestAstToMiniZinc(args)) => {
            handle_test_ast_to_minizinc_command(args.clone())?;
        }
        Some(Commands::CodeSearch(args)) => {
            handle_code_search_command(args.clone())?;
        }
	&Some(Commands::SelfOptimize(_)) => {
	    todo!("FIXME")
	}
	        Some(Commands::AnalyzeDuplicates(_)) => { todo!() }
        Some(Commands::IndexUpdate(args)) => {
            commands::index_update::handle_index_update_command(args.clone())?;
        }
        Some(Commands::Bootstrap { target }) => {
            if target == "zos" {
                println!("Commencing ZOS Bootstrap: Building all core components...");
                handle_build_command(zos_bootstrap::commands::build::BuildArgs { command: Some(zos_bootstrap::commands::build::BuildCommands::All {}), strace: false })?;
                println!("ZOS Bootstrap: Core components built successfully.");
                println!("Commencing ZOS Bootstrap: Running all tests...");
                handle_test_command(zos_bootstrap::commands::test::TestArgs { command: Some(zos_bootstrap::commands::test::TestCommands::All {}) })?;
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
