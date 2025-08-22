// This file contains the process_memes_with_workflow function.
// It processes meme entries in the front matter using a workflow defined by regexes and callbacks.

use std::collections::HashMap;
use anyhow::Result;
use regex::Regex;
use std::path::PathBuf;

//use crate::functions::types::FixedFrontMatter;
use poem_traits::RegexConfig;
use crate::functions::types::PoemFunctionRegistry;
use crate::functions::error_handling::handle_unmatched_regex_error::RegexMatchErrorContext;
use crate::functions::error_handling::handle_unmatched_regex_error::handle_unmatched_regex_error;

pub fn process_memes_with_workflow(
    file_path: &PathBuf,
    meme_lines: &Vec<String>,
    regex_config: &RegexConfig,
    fixed_fm: &mut dyn poem_traits::PoemFrontMatterTrait,
    function_registry: &PoemFunctionRegistry,
    debug_mode: bool,
) -> Result<Vec<String>> {
    let mut matched_regexes: Vec<String> = Vec::new();
    let mut compiled_regexes: HashMap<String, Regex> = HashMap::new();
    for entry in &regex_config.regexes {
        compiled_regexes.insert(entry.name.clone(), Regex::new(&entry.pattern)?);
    }

    for (line_number, line) in meme_lines.iter().enumerate() {
        let mut matched_any_regex = false;
        for entry in &regex_config.regexes {
            if let Some(regex) = compiled_regexes.get(&entry.name) {
                if let Some(captures_raw) = regex.captures(line) {
                    matched_any_regex = true;
                    if debug_mode {
                        println!("  Matched Regex: {}", entry.name);
                        println!("    Line: {line}");
                        println!("    Captures: {captures_raw:?}");
                        println!("    Calling function: {}", entry.callback_function);
                    }
                    matched_regexes.push(entry.name.clone());
                    let captures: Vec<String> = (0..captures_raw.len())
                        .map(|i| captures_raw.get(i).map_or("", |m| m.as_str()).to_string())
                        .collect();

                    if let Some((_metadata, callback)) = function_registry.get(&entry.callback_function) {
                        (callback)(line, captures, fixed_fm)?;
                    } else {
                        // eprintln!("Warning: Callback function '{}' not found in registry for regex '{}'", entry.callback_function, entry.name);
                    }
                    break;
                }
            }
        }
        if !matched_any_regex && !line.trim().is_empty() {
            let context = RegexMatchErrorContext {
                file_path: file_path.to_string_lossy().into_owned(),
                line_number: line_number + 1,
                line_content: line.clone(),
                context_before: if line_number > 0 {
                    meme_lines[0..line_number].iter().rev().take(3).map(|s| s.to_string()).collect::<Vec<String>>().into_iter().rev().collect()
                } else { Vec::new() },
                context_after: meme_lines.iter().skip(line_number + 1).take(3).map(|s| s.to_string()).collect(),
                parsing_state: "processing_meme_lines".to_string(),
                current_tree_path: Vec::new(),
                error_message: format!("No regex matched line: {}", line),
            };
            return handle_unmatched_regex_error(context);
        }
    }
    Ok(matched_regexes)
}
