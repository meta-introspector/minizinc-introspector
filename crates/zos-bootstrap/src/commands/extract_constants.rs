use crate::utils::error::Result;
use crate::utils::subprocess;
use crate::utils::paths;
use crate::code_analysis::string_extractor::{self, ExtractedString};
use crate::code_analysis::constant_usage_proof;
use crate::ExtractConstantsArgs;
//use std::fs;

pub fn handle_extract_constants_command(args: ExtractConstantsArgs) -> Result<()> {
    if args.prove_constants_usage {
        println!("Proving constant usage...");
        let project_root = paths::resolve_project_root()?;
        constant_usage_proof::prove_constants_usage_command(&project_root)?;
        println!("Constant usage proof completed.");
    } else if args.rust_only {
        println!("Extracting constant strings using Rust's syn parser...");
        let project_root = paths::resolve_project_root()?;
        let mut all_extracted_strings: Vec<ExtractedString> = Vec::new();

        // Glob for all .rs files in the project
        let rust_files = glob::glob(&format!("{}/crates/**/*.rs", project_root.display()))? 
            .filter_map(std::result::Result::ok);

        for entry in rust_files {
            let file_path = entry.as_path();
            let crate_name = file_path
                .strip_prefix(&project_root)?
                .components()
                .nth(1)
                .and_then(|c| c.as_os_str().to_str())
                .unwrap_or("unknown_crate")
                .to_string();

            match string_extractor::extract_strings_from_file(file_path, crate_name) {
                Ok(extracted) => {
                    all_extracted_strings.extend(extracted);
                }
                Err(e) => {
                    eprintln!("Error extracting strings from {}: {}", file_path.display(), e);
                }
            }
        }

        if args.generate_sed_script {
            println!("Generating sed script for string constant replacement...");
            for s in all_extracted_strings {
                let constant_name = to_upper_snake_case(&s.string_value);
                let escaped_old_string = escape_for_sed(&s.string_value);
                let new_string = format!("build_constants::{}", constant_name);
                println!("s/{}/{}/g", escaped_old_string, new_string);
            }
            println!("Sed script generation completed.");
        } else {
            println!("\n--- Extracted Strings (Rust Syn Parser) ---");
            for s in all_extracted_strings {
                println!("Crate: {}, Module: {:?}, Function: {:?}, Var: {:?}, Value: \"{}\"",
                           s.crate_name, s.module_path, s.function_name, s.variable_name, s.string_value);
            }
            println!("-------------------------------------------");

            println!("Rust-only constant string extraction completed.");
        }
    } else {
        println!("Extracting constant strings using MiniZinc...");

        let project_root = paths::resolve_project_root()?;
        let minizinc_models_dir = project_root.join("minizinc_models");
        let minizinc_data_dir = project_root.join("minizinc_data");
        let libminizinc_build_dir = paths::get_build_dir()?;
        let _user_solvers_dir = paths::get_minizinc_user_solvers_dir()?;
        println!("MiniZinc user solvers directory: {:?}", _user_solvers_dir);

        let model_file = minizinc_models_dir.join("extract_constants.mzn");
        let data_file = minizinc_data_dir.join("raw_strings_data.dzn");

        let minizinc_exe = libminizinc_build_dir.join("minizinc");

        let args = vec![
            model_file.to_string_lossy().to_string(),
            data_file.to_string_lossy().to_string(),
        ];

        let args_str: Vec<&str> = args.iter().map(|s| s.as_ref()).collect();

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

        println!("Constant string extraction completed.");
    }
    Ok(())
}

fn to_upper_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut last_char_was_uppercase = false;
    let mut last_char_was_digit = false;

    for c in s.chars() {
        if c.is_ascii_alphanumeric() {
            if c.is_ascii_uppercase() {
                if !result.is_empty() && !last_char_was_uppercase && !last_char_was_digit {
                    result.push('_');
                }
                result.push(c);
                last_char_was_uppercase = true;
                last_char_was_digit = false;
            } else if c.is_ascii_lowercase() {
                if !result.is_empty() && last_char_was_uppercase {
                    result.push('_');
                }
                result.push(c.to_ascii_uppercase());
                last_char_was_uppercase = false;
                last_char_was_digit = false;
            } else if c.is_ascii_digit() {
                if !result.is_empty() && !last_char_was_digit {
                    result.push('_');
                }
                result.push(c);
                last_char_was_uppercase = false;
                last_char_was_digit = true;
            }
        } else {
            if !result.is_empty() && result.chars().last() != Some('_') {
                result.push('_');
            }
            last_char_was_uppercase = false;
            last_char_was_digit = false;
        }
    }
    result.trim_matches('_').to_string()
}

fn escape_for_sed(s: &str) -> String {
    let mut escaped_string = String::with_capacity(s.len() * 2); // Pre-allocate for efficiency
    for c in s.chars() {
        match c {
            BACKSLASH => escaped_string.push_str("\\\\"),
            '/' => escaped_string.push_str("\\/"),
            '&' => escaped_string.push_str("\\&"),
            '[' => escaped_string.push_str("\\\["),
            ']' => escaped_string.push_str("\\\]"),
            '(' => escaped_string.push_str("\\("),
            ')' => escaped_string.push_str("\\)"),
            '{' => escaped_string.push_str("\\{{"), // Handle single brace
            '}' => escaped_string.push_str("\\}}"), // Handle single brace
            '<' => escaped_string.push_str("\\<"),
            '>' => escaped_string.push_str("\\>"),
            '|' => escaped_string.push_str("\\|"),
            '

 => escaped_string.push_str("\\"),
            '^' => escaped_string.push_str("\\^"),
            '.' => escaped_string.push_str("\\."),
            '*' => escaped_string.push_str("\\*"),
            '+' => escaped_string.push_str("\\+"),
            '?' => escaped_string.push_str("\\?"),
            '\n' => escaped_string.push_str("\\n"),
            '\r' => escaped_string.push_str("\\r"),
            '\t' => escaped_string.push_str("\\t"),
            _ => escaped_string.push(c),
        }
    }
    escaped_string
}

