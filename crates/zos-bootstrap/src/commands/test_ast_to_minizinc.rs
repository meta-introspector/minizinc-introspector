use clap::Args;
use std::path::PathBuf;
use crate::commands::ast_to_minizinc::AstToMiniZincArgs;
use crate::commands::ast_to_minizinc::handle_ast_to_minizinc_command;
use crate::utils::error::Result;

#[derive(Args, Clone)]
pub struct TestAstToMiniZincArgs {
    /// Path to the Rust file to process.
    #[arg(long)]
    pub file_path: PathBuf,
    /// Enable plan mode to estimate runtime, size, and complexity without full execution.
    #[arg(long)]
    pub plan_mode: bool,
    /// The complexity index for generating the MiniZinc model (e.g., bit size for variables).
    #[arg(long, default_value_t = 2)]
    pub complexity_index: u8,
}

pub fn handle_test_ast_to_minizinc_command(args: TestAstToMiniZincArgs) -> Result<()> {
    println!("--- Testing AST to MiniZinc for single file: {} ---", args.file_path.display());

    let project_root = PathBuf::from("/data/data/com.termux/files/home/storage/github/libminizinc"); // Assuming project root
    let output_dir = project_root.join("minizinc_output_single_file");
    std::fs::create_dir_all(&output_dir)?;

    let ast_to_minizinc_args = AstToMiniZincArgs {
        project_root: project_root.to_string_lossy().to_string(),
        output_dir: output_dir.to_string_lossy().to_string(),
        target_index: None,
        file_subset_index: None, // Process the single file directly
        total_file_subsets: None,
        ast_element_subset_index: None,
        total_ast_element_subsets: None,
        plan_mode: args.plan_mode,
        single_file_path: Some(args.file_path),
        complexity_index: args.complexity_index,
    };

    handle_ast_to_minizinc_command(ast_to_minizinc_args)?;

    println!("--- AST to MiniZinc test for single file completed successfully ---");
    Ok(())
}