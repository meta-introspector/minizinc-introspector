use anyhow::{Result, anyhow};
use poem_traits::PoemFrontMatterTrait;
use std::collections::HashMap;
use regex::Regex; // For regex matching
use poem_traits::{RegexConfig, PoemFunctionMetadata, CallbackFn, FunctionRegistry}; // Import FunctionRegistry

// This function represents the root of the regex-driven YAML fixing process.
// It will use regex matches to determine the state of the parsing/fixing and guide further actions.
// Removed #[poem_macros::poem_function(...)]
#[allow(dead_code)] // This function will be called dynamically
pub fn handle_regex_driven_yaml_fix(
    full_content: &str, // Changed from _line to full_content
    _captures: Vec<String>,
    fixed_fm: &mut dyn PoemFrontMatterTrait,
    regex_config: &RegexConfig, // Pass regex_config
    function_registry: &FunctionRegistry,
) -> Result<(), anyhow::Error> {
    println!("--- Entering Regex-Driven YAML Fixer ---");

    let lines: Vec<&str> = full_content.lines().collect();
    let mut current_line_idx = 0;

    // Skip the initial "---"
    if lines.get(current_line_idx).map_or(false, |l| l.trim() == "---") {
        current_line_idx += 1;
    } else {
        return Err(anyhow!("Expected '---' at the beginning of the file."));
    }

    // Define the expected order of fields and their corresponding regex names
    let expected_fields_order = vec![
        "title_field",
        "summary_field",
        "keywords_field",
        "emojis_field",
        "art_generator_instructions_field",
        // "memes_field", // Memes will be handled specially
        "poem_body_start", // Marks the start of poem_body within YAML
        // "pending_meme_description", // This is handled by meme processing
    ];

    let mut compiled_regexes: HashMap<String, Regex> = HashMap::new();
    for entry in &regex_config.regexes {
        compiled_regexes.insert(entry.name.clone(), Regex::new(&entry.pattern)?);
    }

    // Simulate processing each field in order
    for field_name in expected_fields_order {
        if let Some(regex_entry) = regex_config.regexes.iter().find(|e| e.name == field_name) {
            if let Some(regex) = compiled_regexes.get(&regex_entry.name) {
                // Find the next line that matches this regex
                let mut matched_this_field = false;
                while current_line_idx < lines.len() {
                    let line = lines[current_line_idx];
                    if let Some(captures_raw) = regex.captures(line) {
                        matched_this_field = true;
                        println!("  Matched field: {} with line: {}", field_name, line);

                        let captures: Vec<String> = (0..captures_raw.len())
                            .map(|i| captures_raw.get(i).map_or("", |m| m.as_str()).to_string())
                            .collect();

                        if let Some((_metadata, callback)) = function_registry.get(&regex_entry.callback_function) {
                            (*callback)(line, captures, fixed_fm)?;
                        } else {
                            eprintln!("Warning: Callback function '{}' not found for regex '{}'", regex_entry.callback_function, regex_entry.name);
                        }
                        current_line_idx += 1; // Move to the next line after matching
                        break; // Move to the next expected field
                    }
                    current_line_idx += 1; // Move to the next line if no match
                }
                if !matched_this_field {
                    println!("  Warning: Field '{}' not found or did not match in expected sequence.", field_name);
                    // TODO: Handle missing fields based on schema (e.g., optional vs. required)
                }
            }
        }
    }

    // TODO: Handle memes block and poem_body block after fixed fields
    // This would involve more complex state management and iteration until the closing "---"

    // For demonstration, let's just set a flag or print something
    fixed_fm.set_summary("Regex-driven YAML fixing initiated and conceptual fields processed.".to_string());

    println!("--- Exiting Regex-Driven YAML Fixer ---");
    Ok(())
}
