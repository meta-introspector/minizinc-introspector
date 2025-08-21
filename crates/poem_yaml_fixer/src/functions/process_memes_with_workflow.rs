// This file contains the process_memes_with_workflow function.
// It processes meme entries in the front matter using a workflow defined by regexes and callbacks.

use std::collections::HashMap;
use anyhow::Result;
use regex::{Regex, Captures};
use crate::functions::types::{FixedFrontMatter, RegexConfig, CallbackFn}; // Import types from the types module

pub fn process_memes_with_workflow(
    front_matter_str_for_parsing: &str,
    regex_config: &RegexConfig,
    fixed_fm: &mut FixedFrontMatter,
    function_registry: &HashMap<String, CallbackFn>,
    debug_mode: bool,
) -> Result<()> {
    let mut compiled_regexes: HashMap<String, Regex> = HashMap::new();
    for entry in &regex_config.regexes {
        compiled_regexes.insert(entry.name.clone(), Regex::new(&entry.pattern)?);
    }

    for line in front_matter_str_for_parsing.lines() {
        for entry in &regex_config.regexes {
            if let Some(regex) = compiled_regexes.get(&entry.name) {
                if let Some(captures) = regex.captures(line) {
                    if debug_mode {
                        println!("  Matched Regex: {}", entry.name);
                        println!("    Line: {}", line);
                        println!("    Captures: {:?}", captures);
                        println!("    Calling function: {}", entry.callback_function);
                    }
                    if let Some(callback) = function_registry.get(&entry.callback_function) {
                        callback(line, &captures, fixed_fm)?;
                    } else {
                        eprintln!("Warning: Callback function '{}' not found in registry for regex '{}'", entry.callback_function, entry.name);
                    }
                    // Assuming only one regex matches per line for now, break after first match
                    break;
                }
            }
        }
    }
    Ok(())
}