use anyhow::{Result, anyhow};
use poem_traits::PoemFrontMatterTrait;
use crate::functions::parse_front_matter_with_regex::parse_front_matter_with_regex;
use poem_traits::{RegexConfig, FunctionRegistry}; // Import FunctionRegistry
use std::path::PathBuf;
use crate::functions::process_memes_with_workflow::process_memes_with_workflow;
//use crate::functions::types::RawFrontMatter; // Import RawFrontMatter

// This function represents the root of the regex-driven YAML fixing process.
// It will use regex matches to determine the state of the parsing/fixing and guide further actions.
// Removed #[poem_macros::poem_function(...)]
#[allow(dead_code)] // This function will be called dynamically
pub fn handle_regex_driven_yaml_fix(
    file_path: &PathBuf, // Added file_path
    full_content: &str, // Changed from _line to full_content
    _captures: Vec<String>,
    fixed_fm: &mut dyn PoemFrontMatterTrait,
    regex_config: &RegexConfig, // Pass regex_config
    function_registry: &FunctionRegistry,
) -> Result<(), anyhow::Error> {
    println!("--- Entering Regex-Driven YAML Fixer ---");

    let lines: Vec<&str> = full_content.lines().collect();

    // Skip the initial "---"
    if !lines.get(0).is_some_and(|l| l.trim() == "---") {
        return Err(anyhow!("Expected '---' at the beginning of the file."));
    }

    let front_matter_str = lines.iter().skip(1).take_while(|l| l.trim() != "---").cloned().collect::<Vec<&str>>().join("\n");

    let raw_fm = parse_front_matter_with_regex(&front_matter_str, regex_config, function_registry)?;

    if let Some(title) = raw_fm.title {
        fixed_fm.set_title(title);
    }
    if let Some(summary) = raw_fm.summary {
        fixed_fm.set_summary(summary);
    }
    if let Some(keywords) = raw_fm.keywords {
        fixed_fm.set_keywords(keywords);
    }
    if let Some(emojis) = raw_fm.emojis {
        fixed_fm.set_emojis(emojis);
    }
    if let Some(art_generator_instructions) = raw_fm.art_generator_instructions {
        fixed_fm.set_art_generator_instructions(art_generator_instructions);
    }
    if let Some(poem_body) = raw_fm.poem_body {
        fixed_fm.set_poem_body(poem_body);
    }
    if let Some(pending_meme_description) = raw_fm.pending_meme_description {
        fixed_fm.set_pending_meme_description(pending_meme_description);
    }

    if let Some(raw_meme_lines) = raw_fm.raw_meme_lines {
        let _processed_meme_lines = process_memes_with_workflow(
            file_path,
            &raw_meme_lines,
            regex_config,
            fixed_fm,
            function_registry,
            false, // debug_mode, assuming false for now
        )?;
    }

    println!("--- Exiting Regex-Driven YAML Fixer ---");
    Ok(())
}
