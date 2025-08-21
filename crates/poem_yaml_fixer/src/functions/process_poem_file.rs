// This file contains the core logic for processing a single poem file.
// It orchestrates calls to other functions to extract front matter, parse fields,
// process memes, extract words, and save the word index.

use std::{fs, path::PathBuf, collections::HashMap};
use anyhow::{Result, anyhow};
use serde_yaml;

use crate::{FixedFrontMatter, Meme}; // Assuming FixedFrontMatter and Meme are accessible
use crate::functions::extract_front_matter::extract_front_matter;
use crate::functions::parse_front_matter_fields::parse_front_matter_fields;
use crate::functions::process_memes_with_workflow::process_memes_with_workflow;
use crate::functions::extract_words_from_text::extract_words_from_text;
use crate::functions::save_word_index::{save_word_index, WordIndex};
use crate::functions::create_function_registry::CallbackFn;

// Assuming RegexConfig and RegexEntry are defined in main.rs or a common types module
// For now, we'll re-declare them here for compilation, but ideally they'd be shared.
#[derive(Debug, serde::Deserialize)]
pub struct RegexConfig {
    pub regexes: Vec<RegexEntry>,
}

#[derive(Debug, serde::Deserialize)]
pub struct RegexEntry {
    pub name: String,
    pub pattern: String,
    pub callback_function: String,
}


pub fn process_poem_file(
    path: &PathBuf,
    max_change_percentage: Option<f64>,
    debug_mode: bool,
    regex_config: &RegexConfig, // Pass regex_config
    function_registry: &HashMap<String, CallbackFn>, // Pass function_registry
) -> Result<()> {
    let content = fs::read_to_string(path)?;
    let original_content_len = content.len();
    let mut lines: Vec<&str> = content.lines().collect();

    let (_fm_start, fm_end, front_matter_str_for_parsing, extracted_poem_body_from_fm) = extract_front_matter(&mut lines, &content)?;
    let poem_body_raw_from_file = lines[(fm_end + 1) as usize ..].join("\n");

    let final_poem_body = if !extracted_poem_body_from_fm.is_empty() {
        extracted_poem_body_from_fm
    } else {
        poem_body_raw_from_file
    };

    let mut fixed_fm = FixedFrontMatter {
        title: None,
        summary: None,
        keywords: None,
        emojis: None,
        art_generator_instructions: None,
        memes: Vec::new(),
        poem_body: None,
        pending_meme_description: None,
    };

    parse_front_matter_fields(&front_matter_str_for_parsing, &mut fixed_fm)?;
    process_memes_with_workflow(&front_matter_str_for_parsing, regex_config, &mut fixed_fm, function_registry, debug_mode)?;

    let extracted_words = extract_words_from_text(&final_poem_body);

    if debug_mode {
        println!("\n--- Extracted Words ---");
        println!("{:?}", extracted_words);
        println!("-----------------------");
    }

    let poem_key = fixed_fm.title.clone().unwrap_or_else(|| path.file_stem().unwrap().to_string_lossy().into_owned());

    let word_index_path = PathBuf::from("docs/word_index.yaml");
    let mut word_index: WordIndex = if word_index_path.exists() {
        let content = fs::read_to_string(&word_index_path)?;
        serde_yaml::from_str(&content)?
    } else {
        WordIndex { poems: HashMap::new() }
    };

    word_index.poems.insert(poem_key, extracted_words);
    save_word_index(&word_index, &word_index_path)?;

    if debug_mode {
        println!("\n--- Updated Word Index ---");
        println!("{:?}", word_index);
        println!("--------------------------");
    }

    let mut new_content_parts = Vec::new();
    new_content_parts.push("---".to_string());
    new_content_parts.push(serde_yaml::to_string(&fixed_fm)?);
    new_content_parts.push("---".to_string());

    if let Some(ref pb) = fixed_fm.poem_body {
        new_content_parts.push(pb.clone());
    } else {
        new_content_parts.push(final_poem_body);
    }

    let new_content = new_content_parts.join("\n");
    let new_content_len = new_content.len();

    let change_percentage = (original_content_len as f64 - new_content_len as f64).abs() / original_content_len as f64 * 100.0;
    let effective_max_change = max_change_percentage.unwrap_or(1.0);

    if change_percentage > effective_max_change {
        return Err(anyhow!(
            "Aborting: Content change exceeds allowed limit. Original size: {}, New size: {}, Change: {:.2}%. Max allowed: {:.2}%",
            original_content_len,
            new_content_len,
            change_percentage,
            effective_max_change
        ));
    }

    fs::write(path, new_content)?;

    if debug_mode {
        println!("\n--- Debug Output (Fixed Front Matter) ---");
        println!("{}", serde_yaml::to_string(&fixed_fm)?);
        println!("-----------------------------------------");
    }

    Ok(())
}
