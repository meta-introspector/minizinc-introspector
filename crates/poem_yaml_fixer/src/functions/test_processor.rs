use anyhow::{Result, anyhow};
use regex::Regex;
use std::path::{PathBuf, Path};
use std::fs;
use serde_yaml::Value;
use crate::functions::types::PoemFunctionRegistry;
use crate::functions::process_unmatched_lines_for_grex::process_unmatched_lines_for_grex;

pub fn process_test_yaml(
    test_yaml_path: PathBuf,
    function_registry: &PoemFunctionRegistry,
) -> Result<()> {
    println!("--- Processing Test YAML: {:?} ---", test_yaml_path);

    let content = fs::read_to_string(&test_yaml_path)?;
    let parsed_test_yaml: Value = serde_yaml::from_str(&content)?;

    // Get the first element of the sequence
    let test_data = parsed_test_yaml
        .as_sequence()
        .and_then(|seq| seq.get(0))
        .ok_or_else(|| anyhow!("test YAML is not a sequence or is empty"))?;

    let file_path_str = test_data["file_path"]
        .as_str()
        .ok_or_else(|| anyhow!("file_path not found or not a string in test YAML"))?;
    let file_path = PathBuf::from(file_path_str);

    let unmatched_lines_array = test_data["unmatched_lines"]
        .as_sequence()
        .ok_or_else(|| anyhow!("unmatched_lines not found or not a sequence in test YAML"))?;

    let current_dir = std::env::current_dir()?;
    let log_dir = PathBuf::from("./test_logs");
    fs::create_dir_all(&log_dir)?;

    // Iterate through each unmatched line and try to match it against existing regexes
    for (i, line_value) in unmatched_lines_array.iter().enumerate() {
        let line = line_value
            .as_str()
            .ok_or_else(|| anyhow!("Line in unmatched_lines is not a string"))?;

        println!("\nProcessing unmatched line {}: \"{}\"", i, line);

        let mut line_matched = false;
        for (callback_name, (regex_metadata, _callback_fn)) in function_registry.iter() {
            let regex = Regex::new(&regex_metadata.regex_entry.pattern)?;
            if let Some(_captures) = regex.captures(line) {
                println!("  MATCHED by regex: {} (callback: {})", regex_metadata.regex_entry.name, callback_name);
                line_matched = true;
                break;
            }
        }
        if !line_matched {
            println!("  NO MATCH found for line by existing regexes.");
        }
    }

    // Generate grex regex for all unmatched lines at once
    println!("\nGenerating grex regex for all unmatched lines...");
    process_unmatched_lines_for_grex(
        &unmatched_lines_array.iter().map(|v| v.as_str().unwrap_or("").to_string()).collect::<Vec<String>>(),
        &file_path,
        &current_dir,
        &log_dir,
    )?;

    println!("--- Finished Processing Test YAML ---");
    Ok(())
}

