use crate::utils::error::Result;
use crate::utils::paths;
use crate::utils::subprocess;
use std::path::PathBuf;
use clap::Args;
use crate::code_analysis::ast_to_numerical_vector_converter::{self};
use crate::code_analysis::numerical_vector_to_llm_instructions;
use walkdir::WalkDir;

#[derive(Args, Clone)]
pub struct AstToMiniZincArgs {
    /// Path to the Rust project root directory to analyze.
    #[arg(long)]
    pub project_root: String,
    /// Directory to output the generated MiniZinc model and data files.
    #[arg(long)]
    pub output_dir: String,
}

pub fn handle_ast_to_minizinc_command(args: AstToMiniZincArgs) -> Result<()> {
    println!("\n--- Starting AST to MiniZinc Process ---");
    println!("Analyzing project: {}", args.project_root);

    let project_root_path = PathBuf::from(&args.project_root);
    let output_dir = PathBuf::from(&args.output_dir);
    std::fs::create_dir_all(&output_dir)?;

    let mut all_ast_numerical_vectors = Vec::new();
    let mut processed_files_count = 0;

    println!("\nPhase 1 & 2: Parsing Rust code to AST and extracting numerical vectors...");
    for entry in WalkDir::new(&project_root_path)
        .into_iter()
        .filter_map(|e| e.ok()) // Filter out errors
    {
        let path = entry.path();

        // Process only Rust files.
        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            processed_files_count += 1;
            println!("  Processing file ({}): {}", processed_files_count, path.display());
            let code = std::fs::read_to_string(path)?;
            let syntax = match syn::parse_file(&code) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Warning: Failed to parse Rust file {}: {}", path.display(), e);
                    continue; // Skip this file
                }
            };

            // TODO: Get actual crate name for each file
            let crate_name = path
                .strip_prefix(&project_root_path)?
                .components()
                .nth(1)
                .and_then(|c| c.as_os_str().to_str())
                .unwrap_or("unknown_crate")
                .to_string();

            let ast_numerical_vectors_for_file = ast_to_numerical_vector_converter::convert_ast_to_numerical_vectors(&syntax, crate_name);
            all_ast_numerical_vectors.extend(ast_numerical_vectors_for_file);
        }
    }

    println!("Phase 1 & 2 Complete: Processed {} files.", processed_files_count);
    println!("Extracted {} total AST elements and converted to numerical vectors from the project.", all_ast_numerical_vectors.len());

    println!("\nPhase 3: Generating MiniZinc Data (.dzn)...");
    let data_file_path = output_dir.join("ast_data.dzn");
    let mut dzn_content = String::new();
    dzn_content.push_str("ast_elements_numerical = [
");
    for (i, vec) in all_ast_numerical_vectors.iter().enumerate() {
        dzn_content.push_str(&vec.numerical_vector.to_string());
        if i < all_ast_numerical_vectors.len() - 1 {
            dzn_content.push_str(",\n");
        } else {
            dzn_content.push_str("\n");
        }
    }
    dzn_content.push_str("];\n");

    std::fs::write(&data_file_path, dzn_content)?;
    println!("Phase 3 Complete: Generated MiniZinc data file: {}", data_file_path.display());

    println!("\nPhase 4: Generating MiniZinc Model (.mzn)...");
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
    println!("Phase 4 Complete: Generated MiniZinc model file: {}", model_file_path.display());

    println!("\nPhase 5: Executing MiniZinc...");
    let libminizinc_build_dir = paths::get_build_dir()?;
    let minizinc_exe = libminizinc_build_dir.join("minizinc");

    let args_mzn = vec![
        model_file_path.to_string_lossy().to_string(),
        data_file_path.to_string_lossy().to_string(),
    ];

    let args_str: Vec<&str> = args_mzn.iter().map(|s| s.as_ref()).collect();

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

    println!("Phase 5 Complete: MiniZinc execution finished.");

    println!("\nPhase 6: Parsing MiniZinc Output...");
    let parsed_results = parse_minizinc_output(&String::from_utf8_lossy(&output.stdout))?;
    println!("Phase 6 Complete: MiniZinc Analysis Results ---");
    println!("Suggested Numerical Vector: {}", parsed_results.suggested_numerical_vector);
    println!("-----------------------------------");

    println!("\nPhase 7: Interpreting Solver Output and Generating LLM Instructions...");
    let interpreted_concepts = numerical_vector_to_llm_instructions::interpret_numerical_vector(parsed_results.suggested_numerical_vector);
    let llm_instructions = numerical_vector_to_llm_instructions::generate_llm_instructions(interpreted_concepts);
    println!("Phase 7 Complete: LLM Instructions ---");
    println!("{}", llm_instructions);
    println!("------------------------");

    println!("\n--- AST to MiniZinc Process Completed Successfully ---");
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