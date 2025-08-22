use std::path::PathBuf;
use anyhow::Result;
use std::collections::HashMap;

use crate::functions::types::{WordIndex, PoemFunctionRegistry};
use crate::functions::parse_poem_file_direct;
use crate::functions::process_memes_with_workflow;
use crate::functions::save_word_index;

use crate::functions::extract_words_from_text::extract_words_from_text; // Add this import
use crate::functions::process_poem_file::process_poem_file; // Add this import
use poem_traits::RegexConfig;

pub fn process_single_poem_file_for_report(
    file_path: &PathBuf,
    regex_config: &RegexConfig,
    function_registry: &PoemFunctionRegistry,
    debug_mode: bool,
) -> Result<Vec<String>> {
    println!("Processing for report: {:?}\n", file_path);
    let (mut fixed_fm, poem_body) = parse_poem_file_direct::parse_poem_file_direct(file_path)?;

    // Call extract_words_from_text
    let extracted_words = extract_words_from_text(&poem_body);
    println!("Extracted Words Count: {}", extracted_words.len());
    if debug_mode {
        println!("Extracted Words: {:?}", extracted_words);
    }

    // Call process_memes_with_workflow
    let meme_lines: Vec<String> = poem_body.lines().map(|s| s.to_string()).collect();
    let process_memes_result = process_memes_with_workflow::process_memes_with_workflow(
        file_path,
        &meme_lines,
        regex_config,
        &mut fixed_fm,
        function_registry,
        debug_mode,
    );

    let matched_regexes = if let Ok(m) = process_memes_result {
        m
    } else if let Err(e) = process_memes_result {
        // The error from process_memes_with_workflow is already a rich context error
        // We just need to propagate it.
        return Err(e);
    } else {
        Vec::new()
    };

    // Call process_poem_file (original regex-driven fixer)
    // Run in dry-run mode to avoid actual file changes during reporting
    let process_poem_file_result = process_poem_file(
        file_path,
        Some(90.0), // max_change_percentage (allow large changes for reporting)
        debug_mode,
        true, // dry_run = true
        regex_config,
        function_registry,
    );

    if let Err(e) = process_poem_file_result {
        println!("Warning: Original process_poem_file failed for {:?}: {}", file_path, e);
    } else {
        println!("Original process_poem_file (dry-run) succeeded for {:?}", file_path);
    }

    // Call save_word_index (dummy for now)
    let dummy_word_index = WordIndex {
        poems: HashMap::new(),
    };
    let temp_word_index_path = file_path.with_extension("word_index.yaml");
    save_word_index::save_word_index(&dummy_word_index, &temp_word_index_path)?;

    Ok(matched_regexes)
}
