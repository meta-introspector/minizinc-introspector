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
    std::fs::write(&model_file_path, r###"array[int] of int: ast_elements_numerical;
int: num_elements = length(ast_elements_numerical);

% Define the prime for "security" (from numerical_vector_generator.rs)
int: security_prime = 2;

% Decision variable for the suggested numerical vector
var int: suggested_numerical_vector;

% Constraint: suggested_numerical_vector must be a multiple of security_prime
constraint suggested_numerical_vector mod security_prime = 0;

% Objective: Minimize the absolute difference between the sum of original elements
% and the suggested numerical vector, while satisfying constraints.
% This is a placeholder objective. A more complex objective would involve
% semantic distance or other criteria.
var int: sum_original_elements = sum(ast_elements_numerical);
solve minimize abs(sum_original_elements - suggested_numerical_vector);

output [
    "suggested_numerical_vector = ", show(suggested_numerical_vector), "\n"
];
"###)?;
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

    // Phase 6: Parse MiniZinc Output
    let parsed_results = parse_minizinc_output(&String::from_utf8_lossy(&output.stdout))?;
    println!("\n--- MiniZinc Analysis Results ---");
    println!("Suggested Numerical Vector: {}", parsed_results.suggested_numerical_vector);
    println!("-----------------------------------");

    println!("AST to MiniZinc process completed.");
    Ok(())
}

#[derive(Debug)]
struct MiniZincAnalysisResults {
    suggested_numerical_vector: i32,
}

fn parse_minizinc_output(output_str: &str) -> Result<MiniZincAnalysisResults> {
    let mut suggested_numerical_vector = 0;

    for line in output_str.lines() {
        if line.starts_with("suggested_numerical_vector =") {
            suggested_numerical_vector = line.split("=").nth(1).and_then(|s| s.trim().parse().ok()).unwrap_or(0);
        }
    }

    Ok(MiniZincAnalysisResults {
        suggested_numerical_vector,
    })
}