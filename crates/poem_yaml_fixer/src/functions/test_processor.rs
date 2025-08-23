use anyhow::{Result, anyhow};
use std::path::{PathBuf,
		//Path
};
use std::fs;
use serde_yaml::Value;
use poem_traits::RegexConfig;
use crate::functions::types::PoemFunctionRegistry;
use crate::functions::process_unmatched_lines_for_grex::process_unmatched_lines_for_grex;

pub fn process_test_yaml(
    test_yaml_path: PathBuf,
    regex_config: &RegexConfig,
    function_registry: &PoemFunctionRegistry,
) -> Result<()> {
    println!("--- Processing Test YAML: {:?} ---", test_yaml_path);

    let content = fs::read_to_string(&test_yaml_path)?;
    let parsed_test_yaml: Value = serde_yaml::from_str(&content)?;

    let file_path_str = parsed_test_yaml["file_path"]
        .as_str()
        .ok_or_else(|| anyhow!("file_path not found or not a string in test YAML"))?;
    let file_path = PathBuf::from(file_path_str);

    let unmatched_lines_array = parsed_test_yaml["unmatched_lines"]
        .as_sequence()
        .ok_or_else(|| anyhow!("unmatched_lines not found or not a sequence in test YAML"))?;

    let current_dir = std::env::current_dir()?;
    let log_dir = PathBuf::from("./test_logs");
    fs::create_dir_all(&log_dir)?;

    for (i, line_value) in unmatched_lines_array.iter().enumerate() {
        let line = line_value
            .as_str()
            .ok_or_else(|| anyhow!("Line in unmatched_lines is not a string"))?;

        println!("\nProcessing unmatched line {}: \"{}\"", i, line);

        let mut matched = false;
        for (regex_metadata, callback_fn) in function_registry.iter() {
            let regex = &regex_metadata.regex_entry.regex;
            if let Some(captures) = regex.captures(line) {
                println!("  MATCHED by regex: {}", regex_metadata.name);
                // You can add more detailed reporting here, e.g., captured groups
                matched = true;
                break;
            }
        }

        if !matched {
            println!("  NO MATCH found for line. Generating grex regex...");
            // Call process_unmatched_lines_for_grex for this single line
            process_unmatched_lines_for_grex(
                &[line.to_string()],
                &file_path,
                &current_dir,
                &log_dir,
            )?;
        }
    }

    println!("--- Finished Processing Test YAML ---");
    Ok(())
}

