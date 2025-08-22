//use anyhow::{Result, anyhow};
use poem_traits::PoemFrontMatterTrait;
use poem_traits::{RegexConfig, FunctionRegistry}; // Import FunctionRegistry
use std::path::PathBuf;
use crate::functions::process_document_with_regex::process_document_with_regex;

// This function represents the root of the regex-driven YAML fixing process.
// It will use regex matches to determine the state of the parsing/fixing and guide further actions.
// #[poem_macros::poem_function(
//     name = "handle_regex_driven_yaml_fix",
//     pattern = "a^", // Placeholder pattern that won't match anything
//     title = "Regex Driven YAML Fix",
//     summary = "A top-level function for regex-driven YAML fixing.",
//     keywords = "regex, yaml, fix",
//     emojis = "🛠️✨",
//     art_generator_instructions = "Generate an image of a YAML file being fixed by regex.",
//     pending_meme_description = "This is a pending description for the regex driven YAML fix."
// )]
pub fn handle_regex_driven_yaml_fix(
    file_path: &PathBuf, // Added file_path
    full_content: &str, // Changed from _line to full_content
    _captures: Vec<String>,
    fixed_fm: &mut dyn PoemFrontMatterTrait,
    regex_config: &RegexConfig, // Pass regex_config
    function_registry: &FunctionRegistry,
) -> Result<(), anyhow::Error> {
    println!("--- Entering Regex-Driven YAML Fixer ---");

    let processed_fm = process_document_with_regex(
        file_path,
        full_content,
        regex_config,
        function_registry,
        false, // debug_mode, assuming false for now
    )?;

    // Merge the processed_fm into the fixed_fm passed as argument
    if let Some(title) = processed_fm.title {
        fixed_fm.set_title(title);
    }
    if let Some(summary) = processed_fm.summary {
        fixed_fm.set_summary(summary);
    }
    if let Some(keywords) = processed_fm.keywords {
        fixed_fm.set_keywords(keywords);
    }
    if let Some(emojis) = processed_fm.emojis {
        fixed_fm.set_emojis(emojis);
    }
    if let Some(art_generator_instructions) = processed_fm.art_generator_instructions {
        fixed_fm.set_art_generator_instructions(art_generator_instructions);
    }
    if let Some(memes) = processed_fm.memes {
        fixed_fm.get_memes_mut().extend(memes);
    }
    if let Some(poem_body) = processed_fm.poem_body {
        fixed_fm.set_poem_body(poem_body);
    }
    if let Some(pending_meme_description) = processed_fm.pending_meme_description {
        fixed_fm.set_pending_meme_description(pending_meme_description);
    }

    println!("--- Exiting Regex-Driven YAML Fixer ---");
    Ok(())
}
