// This module contains the logic for parsing front matter with regex.

use anyhow::Result;
#[cfg(test)]
use crate::functions::types::{FixedFrontMatter, RawFrontMatter};
use poem_traits::{RegexConfig, FunctionRegistry};
use std::collections::HashMap;
use regex::Regex;
#[cfg(test)]
use crate::functions::utils::initialize_memes::initialize_memes_option;
use crate::functions::types::RawFrontMatter;
pub fn parse_front_matter_with_regex(
    front_matter: &str,
    regex_config: &RegexConfig,
    _function_registry: &FunctionRegistry,
) -> Result<RawFrontMatter> {
    let mut raw_fm = RawFrontMatter {
        title: None,
        summary: None,
        keywords: None,
        emojis: None,
        art_generator_instructions: None,
        raw_meme_lines: None,
        poem_body: None,
        pending_meme_description: None,
    };

    let lines: Vec<&str> = front_matter.lines().collect();
    let mut current_line_idx = 0;

    // Define the expected order of fields and their corresponding regex names
    let expected_fields_order = vec![
        "title_field",
        "summary_field",
        "keywords_field",
        "emojis_field",
        "art_generator_instructions_field",
        "memes_field", // Memes will be handled specially
        "poem_body_start", // Marks the start of poem_body within YAML
        "pending_meme_description", // This is handled by meme processing
    ];

    let mut compiled_regexes: HashMap<String, Regex> = HashMap::new();
    for entry in &regex_config.regexes {
        compiled_regexes.insert(entry.name.clone(), Regex::new(&entry.pattern)?);
    }

    // Simulate processing each field in order
    for field_name in expected_fields_order {
        let mut matched_this_field = false; // Declare here
        if let Some(regex_entry) = regex_config.regexes.iter().find(|e| e.name == field_name) {
            if let Some(regex) = compiled_regexes.get(&regex_entry.name) {
                if field_name == "memes_field" {
                    let mut meme_lines_buffer: Vec<String> = Vec::new();
                    let mut temp_idx = current_line_idx;
                    // Find the line with "memes:"
                    while temp_idx < lines.len() && lines[temp_idx].trim() != "memes:" {
                        temp_idx += 1;
                    }
                    if temp_idx < lines.len() { // Found "memes:"
                        temp_idx += 1; // Move to the line after "memes:"
                        while temp_idx < lines.len() {
                            let current_line = lines[temp_idx];
                            // Check if the line is indented or empty
                            if current_line.starts_with(" ") || current_line.starts_with("\t") || current_line.trim().is_empty() {
                                meme_lines_buffer.push(current_line.to_string());
                            } else {
                                // End of memes block
                                break;
                            }
                            temp_idx += 1;
                        }
                        raw_fm.raw_meme_lines = Some(meme_lines_buffer);
                        //_current_line_idx = temp_idx; // Update current_line_idx to after the memes block
                        //_matched_this_field = true;
                        println!("  Extracted raw meme lines.");
                        break; // Move to the next expected field
                    }
                }
                // Find the next line that matches this regex
                while current_line_idx < lines.len() {
                    let line = lines[current_line_idx];
                    if let Some(captures_raw) = regex.captures(line) {
                        matched_this_field = true;
                        println!("  Matched field: {field_name} with line: {line}");

                        let captured_value = captures_raw.get(1).map_or("", |m| m.as_str()).to_string();

                        match field_name {
                            "title_field" => raw_fm.title = Some(captured_value),
                            "summary_field" => raw_fm.summary = Some(captured_value),
                            "keywords_field" => raw_fm.keywords = Some(captured_value),
                            "emojis_field" => raw_fm.emojis = Some(captured_value),
                            "art_generator_instructions_field" => raw_fm.art_generator_instructions = Some(captured_value),
                            "poem_body_start" => raw_fm.poem_body = Some(captured_value), // This might need adjustment later for full body
                            "pending_meme_description" => raw_fm.pending_meme_description = Some(captured_value),
                            _ => {},
                        }

                        current_line_idx += 1; // Move to the next line after matching
                        break; // Move to the next expected field
                    }
                    current_line_idx += 1; // Move to the next line if no match
                }
                if !matched_this_field {
                    println!("  Warning: Field '{field_name}' not found or did not match in expected sequence.");
                    // TODO: Handle missing fields based on schema (e.g., optional vs. required)
                }
            }
        }
    }

    Ok(raw_fm)
}
