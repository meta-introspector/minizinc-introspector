use crate::utils::error::Result;
//use crate::code_analysis::string_extractor;
//use crate::code_analysis::minizinc_param_generator;
use crate::code_analysis::string_extractor::ExtractedString;
use crate::code_analysis::string_extractor::extract_strings_from_file;
use crate::code_analysis::minizinc_param_generator::generate_minizinc_data_file;
use crate::code_analysis::minizinc_param_generator::generate_minizinc_selection_model;

use crate::utils::paths;
use crate::utils::subprocess;
use std::path::PathBuf;
use std::str;
use clap::Args;

#[derive(Args, Clone)]
pub struct GenerateParamsArgs {
    #[arg(long)]
    pub output_dir: String,
    #[arg(long)]
    pub input_file: Option<String>, // Optional: path to a file containing extracted strings
}

pub fn handle_generate_params_command(args: GenerateParamsArgs) -> Result<()> {
    println!("Generating MiniZinc parameters...");
    println!("Output directory: {}", args.output_dir);

    let project_root = paths::resolve_project_root()?;
    let mut all_extracted_strings = Vec::new();

    // If an input file is provided, read strings from it (e.g., a list of strings)
    // Otherwise, extract strings from the Rust codebase
    if let Some(input_file_path_str) = args.input_file {
        // For now, assume input_file is a simple text file with one string per line
        // In the future, this could be a JSON file with ExtractedString format
        let input_file_path = PathBuf::from(input_file_path_str);
        let content = std::fs::read_to_string(&input_file_path)?;
        for line in content.lines() {
            if !line.trim().is_empty() {
                all_extracted_strings.push(ExtractedString {
                    crate_name: "manual_input".to_string(),
                    module_path: Vec::new(),
                    function_name: None,
                    variable_name: None,
                    string_value: line.trim().to_string(),
                });
            }
        }
        println!("Extracted {} strings from input file.", all_extracted_strings.len());
    } else {
        // Glob for all .rs files in the project
        let rust_files = glob::glob(&format!("{}/crates/**/*.rs", project_root.display()))?.filter_map(std::result::Result::ok);

        for entry in rust_files {
            let file_path = entry.as_path();
            let crate_name = file_path
                .strip_prefix(&project_root)?
                .components()
                .nth(1)
                .and_then(|c| c.as_os_str().to_str())
                .unwrap_or("unknown_crate")
                .to_string();

            match extract_strings_from_file(file_path, crate_name) {
                Ok(extracted) => {
                    all_extracted_strings.extend(extracted);
                }
                Err(e) => {
                    eprintln!("Error extracting strings from {}: {}", file_path.display(), e);
                }
            }
        }
        println!("Extracted {} strings from Rust codebase.", all_extracted_strings.len());
    }

    let output_dir = PathBuf::from(args.output_dir);
    std::fs::create_dir_all(&output_dir)?;

    // Generate MiniZinc data file
    let data_file = output_dir.join("extracted_strings.dzn");
    generate_minizinc_data_file(all_extracted_strings, &data_file)?;

    // Generate MiniZinc model file
    let model_file = output_dir.join("select_string.mzn");
    generate_minizinc_selection_model(&model_file)?;

    // Execute MiniZinc
    let libminizinc_build_dir = paths::get_build_dir()?;
    let minizinc_exe = libminizinc_build_dir.join("minizinc");

    let args_mzn = vec![
        model_file.to_string_lossy().to_string(),
        data_file.to_string_lossy().to_string(),
    ];

    let args_str: Vec<&str> = args_mzn.iter().map(|s| s.as_ref()).collect();

    println!("Running MiniZinc to select a constant...");
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

    // Parse MiniZinc output to get the selected string
    let output_str = str::from_utf8(&output.stdout)?;
    let selected_string_prefix = "selected_string = \"";
    if let Some(start_index) = output_str.find(selected_string_prefix) {
        let remaining = &output_str[start_index + selected_string_prefix.len()..];
        if let Some(end_index) = remaining.find("\";") {
            let selected_string = &remaining[..end_index];
            println!("Selected constant: {}", selected_string);
        } else {
            println!("Could not parse selected string from MiniZinc output.");
        }
    } else {
        println!("Could not find selected string in MiniZinc output.");
    }

    println!("MiniZinc parameter generation and selection completed.");
    Ok(())
}

