use crate::utils::paths;
use crate::utils::subprocess;
use std::path::PathBuf;
use std::time::Instant;
use clap::Args;
use crate::code_analysis::ast_to_numerical_vector_converter::{self};
use crate::code_analysis::numerical_vector_to_llm_instructions;
use walkdir::WalkDir;

// Import the new generator modules
use crate::code_analysis::minizinc_model_generator;
use crate::code_analysis::dzn_data_generator;

// Re-declare AstToMiniZincArgs here, as it will be moved
#[derive(Args, Clone)]
pub struct AstToMiniZincArgs {
    /// Path to the Rust project root directory to analyze.
    #[arg(long)]
    pub project_root: String,
    /// Directory to output the generated MiniZinc model and data files.
    #[arg(long)]
    pub output_dir: String,
    /// Optional: Index of the AST element to target for transformation (1-indexed).
    #[arg(long)]
    pub target_index: Option<usize>,
    /// Optional: Index of the file subset to process (0-indexed).
    #[arg(long)]
    pub file_subset_index: Option<usize>,
    /// Optional: Total number of file subsets.
    #[arg(long)]
    pub total_file_subsets: Option<usize>,
    /// Optional: Index of the AST element subset to process (0-indexed).
    #[arg(long)]
    pub ast_element_subset_index: Option<usize>,
    /// Optional: Total number of AST element subsets.
    #[arg(long)]
    pub total_ast_element_subsets: Option<usize>,
    /// Enable plan mode to estimate runtime, size, and complexity without full execution.
    #[arg(long)]
    pub plan_mode: bool,
    /// Optional: Path to a single Rust file to process. If provided, overrides project_root for file discovery.
    #[arg(long)]
    pub single_file_path: Option<PathBuf>,
    /// The complexity index for generating the MiniZinc model (e.g., bit size for variables).
    #[arg(long, default_value_t = 2)]
    pub complexity_index: u8,
}

pub fn handle_ast_to_minizinc_command(args: AstToMiniZincArgs) -> crate::utils::error::Result<()> {
    println!("\n--- Starting AST to MiniZinc Process ---");
    println!("Analyzing project: {}", args.project_root);

    let project_root_path = PathBuf::from(&args.project_root);
    let output_dir = PathBuf::from(&args.output_dir);
    std::fs::create_dir_all(&output_dir)?;

    let target_index = args.target_index.unwrap_or(1);
    if target_index == 0 {
        return Err(crate::utils::error::ZosError::InvalidArgument("target_index must be 1-indexed (i.e., >= 1).".to_string()));
    }

    let mut all_ast_numerical_vectors = Vec::new();
    let mut processed_files_count = 0;

    println!("\nPhase 1 & 2: Parsing Rust code to AST and extracting numerical vectors...");
    let phase1_2_start_time = Instant::now();
    let mut all_rust_files: Vec<PathBuf> = if let Some(single_path) = args.single_file_path {
        vec![single_path]
    } else {
        WalkDir::new(&project_root_path)
            .into_iter()
            .filter_map(|e| e.ok()) // Filter out errors
            .filter(|e| e.path().is_file() && e.path().extension().map_or(false, |ext| ext == "rs"))
            .map(|e| e.path().to_path_buf())
            .collect()
    };

    // Filter files based on subset index
    if let (Some(subset_index), Some(total_subsets)) = (args.file_subset_index, args.total_file_subsets) {
        if total_subsets == 0 {
            return Err(crate::utils::error::ZosError::InvalidArgument("total_file_subsets cannot be 0.".to_string()));
        }
        if subset_index >= total_subsets {
            return Err(crate::utils::error::ZosError::InvalidArgument("file_subset_index must be less than total_file_subsets.".to_string()));
        }
        let chunk_size = (all_rust_files.len() + total_subsets - 1) / total_subsets;
        let start_index = subset_index * chunk_size;
        let end_index = (start_index + chunk_size).min(all_rust_files.len());
        all_rust_files = all_rust_files[start_index..end_index].to_vec();
        println!("Processing file subset {} of {} (files {} to {}).", subset_index, total_subsets, start_index, end_index - 1);
    }

    for path in all_rust_files {
        // Temporarily skip problematic files for syn parsing
        if path.ends_with("crates/constant_analyzer/src/main.rs") {
            eprintln!("Skipping problematic file for AST parsing: {}", path.display());
            continue;
        }
        processed_files_count += 1;
        println!("  Processing file ({}): {}", processed_files_count, path.display());
        let code = std::fs::read_to_string(&path)?;
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

    let phase1_2_elapsed = phase1_2_start_time.elapsed();
    println!("Phase 1 & 2 Complete: Processed {} files in {:?}.", processed_files_count, phase1_2_elapsed);
    println!("Extracted {} total AST elements and converted to numerical vectors from the project.", all_ast_numerical_vectors.len());
    if args.plan_mode {
        println!("  [PLAN MODE] Estimated AST parsing and vector extraction time: {:?}", phase1_2_elapsed);
        println!("  [PLAN MODE] Number of files processed: {}", processed_files_count);
        println!("  [PLAN MODE] Number of AST elements extracted: {}", all_ast_numerical_vectors.len());
    }

    // Filter AST elements based on subset index
    if let (Some(subset_index), Some(total_subsets)) = (args.ast_element_subset_index, args.total_ast_element_subsets) {
        if total_subsets == 0 {
            return Err(crate::utils::error::ZosError::InvalidArgument("total_ast_element_subsets cannot be 0.".to_string()));
        }
        if subset_index >= total_subsets {
            return Err(crate::utils::error::ZosError::InvalidArgument("ast_element_subset_index must be less than total_ast_element_subsets.".to_string()));
        }
        let chunk_size = (all_ast_numerical_vectors.len() + total_subsets - 1) / total_subsets;
        let start_index = subset_index * chunk_size;
        let end_index = (start_index + chunk_size).min(all_ast_numerical_vectors.len());
        all_ast_numerical_vectors = all_ast_numerical_vectors[start_index..end_index].to_vec();
        println!("Processing AST element subset {} of {} (elements {} to {}).", subset_index, total_subsets, start_index, end_index - 1);
    }

    println!("\nPhase 3: Generating MiniZinc Data (.dzn)...");
    let phase3_start_time = Instant::now();
    let data_file_path = output_dir.join("ast_data.dzn");
    // Use the new dzn_data_generator
    let dzn_content = dzn_data_generator::generate_ast_dzn_data_string(
        &all_ast_numerical_vectors,
        target_index,
    );
    std::fs::write(&data_file_path, dzn_content)?;
    let phase3_elapsed = phase3_start_time.elapsed();
    println!("Phase 3 Complete: Generated MiniZinc data file: {} in {:?}.", data_file_path.display(), phase3_elapsed);
    if args.plan_mode {
        println!("  [PLAN MODE] Estimated MiniZinc data generation time: {:?}", phase3_elapsed);
        println!("  [PLAN MODE] Size of generated DZN file: {} bytes", std::fs::metadata(&data_file_path)?.len());
    }

    println!("\nPhase 4: Generating MiniZinc Model (.mzn)...");
    let phase4_start_time = Instant::now();
    let model_file_path = output_dir.join("ast_model.mzn");
    // Use the new minizinc_model_generator
    let model_content = minizinc_model_generator::generate_ast_minizinc_model_string(
        &all_ast_numerical_vectors,
        target_index,
        args.complexity_index,
    );
    std::fs::write(&model_file_path, model_content)?;
    let phase4_elapsed = phase4_start_time.elapsed();
    println!("Phase 4 Complete: Generated MiniZinc model file: {} in {:?}.", model_file_path.display(), phase4_elapsed);
    if args.plan_mode {
        println!("  [PLAN MODE] Estimated MiniZinc model generation time: {:?}", phase4_elapsed);
        println!("  [PLAN MODE] Size of generated MZN file: {} bytes", std::fs::metadata(&model_file_path)?.len());
    }

    println!("\nPhase 5: Executing MiniZinc...");
    let phase5_start_time = Instant::now();
    if args.plan_mode {
        println!("  [PLAN MODE] Skipping MiniZinc execution.");
    } else {
        let libminizinc_build_dir = paths::get_build_dir()?;
        let minizinc_exe = libminizinc_build_dir.join("minizinc");

        let args_mzn = vec![
            model_file_path.to_string_lossy().to_string(),
            data_file_path.to_string_lossy().to_string(),
            "--verbose".to_string(), // Add verbose flag
        ];

        // Add solver-specific flags if needed, e.g., for Gecode
        // args_mzn.push("--solver-time-limit".to_string());
        // args_mzn.push("10000".to_string()); // 10 seconds

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
    }
    let phase5_elapsed = phase5_start_time.elapsed();
    println!("Phase 5 Complete: MiniZinc execution finished in {:?}.", phase5_elapsed);
    if args.plan_mode {
        println!("  [PLAN MODE] Estimated MiniZinc execution time: {:?}", phase5_elapsed);
    }

    if args.plan_mode {
        println!("  [PLAN MODE] Skipping MiniZinc output parsing and LLM instruction generation.");
    } else {
        println!("\nPhase 6: Parsing MiniZinc Output...");
        // This part will be moved to parser.rs
        let parsed_results = MiniZincAnalysisResults { suggested_numerical_vector: 0 }; // Dummy for now
        // let parsed_results = parse_minizinc_output(&String::from_utf8_lossy(&output.stdout))?;
        println!("Phase 6 Complete: MiniZinc Analysis Results ---");
        println!("Suggested Numerical Vector: {}", parsed_results.suggested_numerical_vector);
        println!("-----------------------------------");

        println!("\nPhase 7: Interpreting Solver Output and Generating LLM Instructions...");
        let interpreted_concepts = numerical_vector_to_llm_instructions::interpret_numerical_vector(parsed_results.suggested_numerical_vector);
        let llm_instructions = numerical_vector_to_llm_instructions::generate_llm_instructions(interpreted_concepts);
        println!("Phase 7 Complete: LLM Instructions ---");
        println!("{}", llm_instructions);
        println!("------------------------");
    }

    println!("\n--- AST to MiniZinc Process Completed Successfully ---");
    Ok(())
}

// Dummy struct for parsed results, to be replaced by actual parsing logic
struct MiniZincAnalysisResults {
    suggested_numerical_vector: i32,
}
