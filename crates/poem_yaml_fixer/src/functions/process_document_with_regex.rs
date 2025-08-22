use crate::FixedFrontMatter;
//use anyhow::{Result, anyhow};
//use poem_traits::PoemFrontMatterTrait;
use poem_traits::{RegexConfig, FunctionRegistry};
use std::path::PathBuf;
//use crate::functions::process_memes_with_workflow::process_memes_with_workflow;
//use crate::functions::types::{FixedFrontMatter, RawFrontMatter};
use regex::Regex;
use std::collections::HashMap;
//use crate::functions::callbacks::handle_new_document::handle_new_document;
use crate::functions::regex_patterns::DOCUMENT_SEPARATOR_REGEX_PATTERN;

pub fn process_document_with_regex(
    file_path: &PathBuf,
    full_content: &str,
    regex_config: &RegexConfig,
    function_registry: &FunctionRegistry,
    debug_mode: bool,
) -> Result<FixedFrontMatter> {
    let mut stack: Vec<FixedFrontMatter> = vec![FixedFrontMatter::default()];

    let compiled_regexes: HashMap<String, Regex> = regex_config
        .regexes
        .iter()
        .map(|entry| (entry.name.clone(), Regex::new(&entry.pattern).unwrap()))
        .collect();

    for line in full_content.lines() {
        let mut matched = false;
        // Check for document separator
        if line.trim() == DOCUMENT_SEPARATOR_REGEX_PATTERN {
            // handle_new_document(line, Vec::new(), fixed_fm)?; // fixed_fm is not in scope here
            stack.push(FixedFrontMatter::default());
            matched = true;
        }

        if !matched {
            // Try to match other regexes
            for regex_entry in &regex_config.regexes {
                if let Some(regex) = compiled_regexes.get(&regex_entry.name) {
                    if let Some(captures_raw) = regex.captures(line) {
                        matched = true;
                        println!("  Matched field: {} with line: {}", regex_entry.name, line);

                        let captures: Vec<String> = (0..captures_raw.len())
                            .map(|i| captures_raw.get(i).map_or("", |m| m.as_str()).to_string())
                            .collect();

                        if let Some(current_fm) = stack.last_mut() {
                            if let Some((_metadata, callback)) = function_registry.get(&regex_entry.callback_function) {
                                (*callback)(line, captures, current_fm)?;
                            } else {
                                eprintln!("Warning: Callback function '{}' not found for regex '{}'", regex_entry.callback_function, regex_entry.name);
                            }
                        }
                        break; // Only match one regex per line
                    }
                }
            }
        }

        if !matched {
            // If no regex matched, treat as poem body
            if let Some(current_fm) = stack.last_mut() {
                let mut body = current_fm.poem_body.take().unwrap_or_default();
                body.push_str(line);
                body.push('\n');
                current_fm.poem_body = Some(body);
            }
        }
    }

    // Return the last FixedFrontMatter from the stack
    Ok(stack.pop().unwrap_or_default())
}
