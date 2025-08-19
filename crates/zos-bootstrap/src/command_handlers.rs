use crate::cli::{Cli, Commands};
use clap::Parser; // Import Parser trait
use crate::utils::error::{Result, ZosError}; // Assuming ZosError is in utils::error
use crate::commands;
use crate::code_analysis; // Import the code_analysis module
use std::fs; // Import fs for file operations
use std::path::PathBuf; // Import PathBuf

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
        Some(Commands::TestAstToMiniZinc(args)) => commands::test_ast_to_minizinc::handle_test_ast_to_minizinc_command(args).map_err(|e| ZosError::Unknown(format!("TestAstToMiniZinc command failed: {}", e))),
        Some(Commands::AnalyzeDuplicates(args)) => {
            // New match arm for AnalyzeDuplicates
            let suggested_code_content = if args.is_file {
                fs::read_to_string(&args.suggested_code)
                    .map_err(|e| ZosError::Unknown(format!("Failed to read suggested code from file {}: {}", args.suggested_code.display(), e)))
            ? {
                args.suggested_code
            };

            println!("Analyzing for duplicates in: {}", args.search_path.display());

            match code_analysis::find_duplicate_code(&suggested_code_content, &args.search_path) {
                Ok(matches) => {
                    if matches.is_empty() {
                        println!("No similar code found.");
                    } else {
                        println!("\nFound similar code matches:");
                        for (i, m) in matches.iter().enumerate() {
                            println!("---"Match {}" ---", i + 1);
                            println!("File: {}", m.file_path.display());
                            println!("Similarity Score: {:.4}", m.similarity_score);
                            println!("Code Snippet (first 10 lines):\n{}", m.code_snippet.lines().take(10).collect::<Vec<&str>>().join("\n"));
                            println!("-----------------\n");
                        }
                    }
                    Ok(())
                }
                Err(e) => {
                    Err(ZosError::Unknown(format!("Code analysis failed: {:?}", e)))
                }
            }
        },
        None => {
            println!("No command provided. Use --help for more information.");
            Ok(())
        }
    }
}