use crate::utils::error::Result;
use crate::utils::paths;
use crate::utils::subprocess;
use std::path::PathBuf;
use clap::Args;
use crate::code_analysis::ast_to_numerical_vector_converter::{self};

#[derive(Args, Clone)]
pub struct AstToMiniZincArgs {
    /// Path to the Rust file to analyze.
    #[arg(long)]
    pub file_path: String,
    /// Directory to output the generated MiniZinc model and data files.
    #[arg(long)]
    pub output_dir: String,
}

pub fn handle_ast_to_minizinc_command(args: AstToMiniZincArgs) -> Result<()> {
    println!("Analyzing AST and generating MiniZinc files for: {}", args.file_path);

    let input_file_path = PathBuf::from(&args.file_path);
    let output_dir = PathBuf::from(&args.output_dir);
    std::fs::create_dir_all(&output_dir)?;

    // Phase 1: Parse Rust code to AST
    let code = std::fs::read_to_string(&input_file_path)?;
    let syntax = syn::parse_file(&code)?;

    // Phase 2: Extract relevant AST data and convert to numerical vectors
    let ast_numerical_vectors = ast_to_numerical_vector_converter::convert_ast_to_numerical_vectors(&syntax, "my_crate".to_string()); // TODO: Get actual crate name
    println!("Extracted {} AST elements and converted to numerical vectors.", ast_numerical_vectors.len());

    // Phase 3: Generate MiniZinc Data (.dzn)
    let data_file_path = output_dir.join("ast_data.dzn");
    let mut dzn_content = String::new();
    dzn_content.push_str("ast_elements_numerical = [
");
    for (i, vec) in ast_numerical_vectors.iter().enumerate() {
        dzn_content.push_str(&vec.numerical_vector.to_string());
        if i < ast_numerical_vectors.len() - 1 {
            dzn_content.push_str(",\n");
        } else {
            dzn_content.push_str("\n");
        }
    }
    dzn_content.push_str("];\n");

    std::fs::write(&data_file_path, dzn_content)?;
    println!("Generated MiniZinc data file: {}", data_file_path.display());

    // Phase 4: Generate MiniZinc Model (.mzn)
    let model_file_path = output_dir.join("ast_model.mzn");
    // This will contain the MiniZinc model for AST analysis/transformation
    std::fs::write(&model_file_path, "array[int] of int: ast_elements_numerical;\nvar int: x; solve satisfy; output [\"x = \", show(x), \"\n\"];\n")?;
    println!("Generated MiniZinc model file: {}", model_file_path.display());

    // Phase 5: Execute MiniZinc
    let libminizinc_build_dir = paths::get_build_dir()?;
    let minizinc_exe = libminizinc_build_dir.join("minizinc");

    let args_mzn = vec![
        model_file_path.to_string_lossy().to_string(),
        data_file_path.to_string_lossy().to_string(),
    ];

    let args_str: Vec<&str> = args_mzn.iter().map(|s| s.as_ref()).collect();

    println!("Running MiniZinc for AST analysis...");
    let output = subprocess::run_command(&minizinc_exe.to_string_lossy(), &args_str)?;

    println!("MiniZinc Output:\n{}", String::from_utf8_lossy(&output.stdout));
    println!("MiniZinc Errors:\n{}", String::from_utf8_lossy(&output.stderr));

    if !output.status.success() {
        return Err(crate::utils::error::ZosError::CommandFailed {
            command: format!("minizinc {}", args_str.join(" ")),
            exit_code: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        });
    }

    println!("AST to MiniZinc process completed.");
    Ok(())
}
