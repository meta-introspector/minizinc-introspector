// This file contains the core logic for processing a single poem file.
// It orchestrates calls to other functions to extract front matter, parse fields,
// process memes, extract words, and save the word index.

use std::{fs, path::PathBuf, collections::HashMap};
use anyhow::{Result, anyhow};
use serde_yaml;

use crate::functions::types::{FixedFrontMatter,
			      RegexConfig,
			      WordIndex}; // Import types from the types module
use poem_traits::{CallbackFn, PoemFrontMatterTrait};
use crate::functions::extract_front_matter::extract_front_matter;
use crate::functions::parse_front_matter_fields::parse_front_matter_fields;
use crate::functions::process_memes_with_workflow::process_memes_with_workflow;
use crate::functions::extract_words_from_text::extract_words_from_text;

use crate::functions::save_word_index::{save_word_index};

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

    let parsed_front_matter: serde_yaml::Value = serde_yaml::from_str(&front_matter_str_for_parsing)
        .map_err(|e| anyhow!("Failed to parse front matter YAML: {}", e))?;

    if let Some(title) = parsed_front_matter.get("title").and_then(|v| v.as_str()) {
        fixed_fm.set_title(title.to_string());
    }
    if let Some(summary) = parsed_front_matter.get("summary").and_then(|v| v.as_str()) {
        fixed_fm.set_summary(summary.to_string());
    }
    if let Some(keywords) = parsed_front_matter.get("keywords").and_then(|v| v.as_str()) {
        fixed_fm.set_keywords(keywords.to_string());
    }
    if let Some(emojis) = parsed_front_matter.get("emojis").and_then(|v| v.as_str()) {
        fixed_fm.set_emojis(emojis.to_string());
    }
    if let Some(art_generator_instructions) = parsed_front_matter.get("art_generator_instructions").and_then(|v| v.as_str()) {
        fixed_fm.set_art_generator_instructions(art_generator_instructions.to_string());
    }
    let mut new_content_parts = Vec::new();
    new_content_parts.push("---".to_string());
    new_content_parts.push(serde_yaml::to_string(&fixed_fm)?);

    // Handle poem_body formatting
    if let Some(pb) = fixed_fm.poem_body.take() {
        new_content_parts.push("poem_body: |".to_string());
        for line in pb.lines() {
            new_content_parts.push(format!("  {}", line)); // Indent each line
        }
    } else {
        new_content_parts.push(final_poem_body);
    }

    new_content_parts.push("---".to_string());

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
