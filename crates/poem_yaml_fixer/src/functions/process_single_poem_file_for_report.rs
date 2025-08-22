use std::path::PathBuf;
use anyhow::Result;
use std::collections::HashMap;

use crate::functions::types::{WordIndex, PoemFunctionRegistry};
use crate::functions::parse_poem_file_direct;
use crate::functions::process_memes_with_workflow;
use crate::functions::save_word_index;
use crate::functions::error_handling::handle_unmatched_regex_error::handle_unmatched_regex_error;
use poem_traits::RegexConfig;

pub fn process_single_poem_file_for_report(
    file_path: &PathBuf,
    regex_config: &RegexConfig,
    function_registry: &PoemFunctionRegistry,
    debug_mode: bool,
) -> Result<Vec<String>> {
    println!("Processing for report: {:?}\n", file_path);
    let (mut fixed_fm, poem_body) = parse_poem_file_direct::parse_poem_file_direct(file_path)?;

    // Call process_memes_with_workflow
    let meme_lines: Vec<String> = poem_body.lines().map(|s| s.to_string()).collect();
    let process_memes_result = process_memes_with_workflow::process_memes_with_workflow(
        &meme_lines,
        regex_config,
        &mut fixed_fm,
        function_registry,
        debug_mode,
    );

    let matched_regexes = if let Ok(m) = process_memes_result {
        m
    } else if let Err(e) = process_memes_result {
        let error_msg = format!("Error processing memes: {}", e);
        handle_unmatched_regex_error(file_path, &error_msg)?;
        Vec::new()
    } else {
        Vec::new()
    };

    // Call save_word_index (dummy for now)
    let dummy_word_index = WordIndex {
        poems: HashMap::new(),
    };
    let temp_word_index_path = file_path.with_extension("word_index.yaml");
    save_word_index::save_word_index(&dummy_word_index, &temp_word_index_path)?;

    Ok(matched_regexes)
}
