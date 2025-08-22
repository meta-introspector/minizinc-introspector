// This file contains the parse_front_matter_fields function.
// It is responsible for parsing basic front matter fields (title, summary, etc.)
// from a YAML string and updating the FixedFrontMatter struct.

use anyhow::Result;
use crate::functions::types::FixedFrontMatter; // Import FixedFrontMatter from types module
use regex::Regex;
use std::collections::HashMap;
use poem_traits::{RegexConfig, FunctionRegistry, PoemFrontMatterTrait};
//use lazy_static::lazy_static; // For compiling regexes once

#[allow(dead_code)]
pub fn parse_front_matter_fields(
    front_matter_str_for_parsing: &str,
    fixed_fm: &mut FixedFrontMatter,
    regex_config: &RegexConfig, // Add regex_config
    _function_registry: &FunctionRegistry, // Add function_registry
) -> Result<()> {
    let lines: Vec<&str> = front_matter_str_for_parsing.lines().collect();

    // Compile regexes from config for direct use
    let mut compiled_regexes: HashMap<String, Regex> = HashMap::new();
    for entry in &regex_config.regexes {
        compiled_regexes.insert(entry.name.clone(), Regex::new(&entry.pattern)?);
    }

    // Process each line to extract fields
    for line in lines {
        // Title
        if let Some(regex_entry) = regex_config.regexes.iter().find(|e| e.name == "title_regex") {
            if let Some(regex) = compiled_regexes.get(&regex_entry.name) {
                if let Some(captures) = regex.captures(line) {
                    let title = captures.get(1).map_or("", |m| m.as_str()).to_string();
                    fixed_fm.set_title(title);
                }
            }
        }

        // Summary
        if let Some(regex_entry) = regex_config.regexes.iter().find(|e| e.name == "summary_regex") {
            if let Some(regex) = compiled_regexes.get(&regex_entry.name) {
                if let Some(captures) = regex.captures(line) {
                    let summary = captures.get(1).map_or("", |m| m.as_str()).to_string();
                    fixed_fm.set_summary(summary);
                }
            }
        }

        // Keywords (special handling for Vec<String>)
        if let Some(regex_entry) = regex_config.regexes.iter().find(|e| e.name == "keywords_regex") {
            if let Some(regex) = compiled_regexes.get(&regex_entry.name) {
                if let Some(captures) = regex.captures(line) {
                    let keywords_str = captures.get(1).map_or("", |m| m.as_str()).to_string();
                    // Assuming keywords are comma-separated in the captured string
                    let keywords_vec: Vec<String> = keywords_str
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                    fixed_fm.keywords = Some(keywords_vec);
                }
            }
        }

        // Emojis
        if let Some(regex_entry) = regex_config.regexes.iter().find(|e| e.name == "emojis_regex") {
            if let Some(regex) = compiled_regexes.get(&regex_entry.name) {
                if let Some(captures) = regex.captures(line) {
                    let emojis = captures.get(1).map_or("", |m| m.as_str()).to_string();
                    fixed_fm.set_emojis(emojis);
                }
            }
        }

        // Art Generator Instructions
        if let Some(regex_entry) = regex_config.regexes.iter().find(|e| e.name == "art_generator_instructions_regex") {
            if let Some(regex) = compiled_regexes.get(&regex_entry.name) {
                if let Some(captures) = regex.captures(line) {
                    let instructions = captures.get(1).map_or("", |m| m.as_str()).to_string();
                    fixed_fm.set_art_generator_instructions(instructions);
                }
            }
        }
    }

    Ok(())
}
