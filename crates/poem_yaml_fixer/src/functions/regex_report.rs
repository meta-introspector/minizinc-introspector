use anyhow::{Result, anyhow};
use std::fs;
use toml::Value;
use regex::Regex;

pub fn generate_regex_report() -> Result<()> {
    println!("--- Regex Report ---");

    let config_path = "crates/poem_yaml_fixer/src/regex_patterns.toml";
    println!("Reading regex patterns from: {}", config_path);
    let toml_content = fs::read_to_string(config_path)?;
    let parsed_toml: Value = toml::from_str(&toml_content)?;

    let regexes_array = parsed_toml
        .get("regexes")
        .and_then(|v| v.as_array())
        .ok_or_else(|| anyhow!("'regexes' array not found in regex_patterns.toml"))?;

    for (i, regex_entry) in regexes_array.iter().enumerate() {
        let name = regex_entry
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("N/A");
        let pattern = regex_entry
            .get("pattern")
            .and_then(|v| v.as_str())
            .unwrap_or("N/A");
        let callback_function = regex_entry
            .get("callback_function")
            .and_then(|v| v.as_str())
            .unwrap_or("N/A");

        println!("\nEntry {}:", i);
        println!("  Name: {}", name);
        println!("  Pattern: {}", pattern);
        println!("  Callback Function: {}", callback_function);

        match Regex::new(pattern) {
            Ok(_) => println!("  Compilation Status: SUCCESS"),
            Err(e) => println!("  Compilation Status: FAILED - {}", e),
        }
    }

    println!("\n--- End of Regex Report ---");
    Ok(())
}
