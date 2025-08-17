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

var int: sum_elements = sum(ast_elements_numerical);
var int: min_element = min(ast_elements_numerical);
var int: max_element = max(ast_elements_numerical);

solve satisfy;

output [
    "num_elements = ", show(num_elements), "\n",
    "sum_elements = ", show(sum_elements), "\n",
    "min_element = ", show(min_element), "\n",
    "max_element = ", show(max_element), "\n"
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
    println!("Number of elements: {}", parsed_results.num_elements);
    println!("Sum of elements: {}", parsed_results.sum_elements);
    println!("Min element: {}", parsed_results.min_element);
    println!("Max element: {}", parsed_results.max_element);
    println!("-----------------------------------");

    println!("AST to MiniZinc process completed.");
    Ok(())
}

#[derive(Debug)]
struct MiniZincAnalysisResults {
    num_elements: i32,
    sum_elements: i32,
    min_element: i32,
    max_element: i32,
}

fn parse_minizinc_output(output_str: &str) -> Result<MiniZincAnalysisResults> {
    let mut num_elements = 0;
    let mut sum_elements = 0;
    let mut min_element = 0;
    let mut max_element = 0;

    for line in output_str.lines() {
        if line.starts_with("num_elements =") {
            num_elements = line.split("=").nth(1).and_then(|s| s.trim().parse().ok()).unwrap_or(0);
        } else if line.starts_with("sum_elements =") {
            sum_elements = line.split("=").nth(1).and_then(|s| s.trim().parse().ok()).unwrap_or(0);
        } else if line.starts_with("min_element =") {
            min_element = line.split("=").nth(1).and_then(|s| s.trim().parse().ok()).unwrap_or(0);
        } else if line.starts_with("max_element =") {
            max_element = line.split("=").nth(1).and_then(|s| s.trim().parse().ok()).unwrap_or(0);
        }
    }

    Ok(MiniZincAnalysisResults {
        num_elements,
        sum_elements,
        min_element,
        max_element,
    })
}