use anyhow::{Result, anyhow};
use poem_traits::PoemFrontMatterTrait;
use crate::functions::parse_front_matter_with_regex::parse_front_matter_with_regex;
use poem_traits::{RegexConfig, FunctionRegistry}; // Import FunctionRegistry

// This function represents the root of the regex-driven YAML fixing process.
// It will use regex matches to determine the state of the parsing/fixing and guide further actions.
// Removed #[poem_macros::poem_function(...)]
#[allow(dead_code)] // This function will be called dynamically
pub fn handle_regex_driven_yaml_fix(
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

    let parsed_fm = parse_front_matter_with_regex(&front_matter_str, regex_config, function_registry)?;

    if let Some(title) = parsed_fm.title {
        fixed_fm.set_title(title);
    }
    if let Some(summary) = parsed_fm.summary {
        fixed_fm.set_summary(summary);
    }
    // if let Some(keywords) = parsed_fm.keywords { // Commented out: Handled by callback
    //     fixed_fm.set_keywords(keywords);
    // }
    if let Some(emojis) = parsed_fm.emojis {
        fixed_fm.set_emojis(emojis);
    }
    if let Some(art_generator_instructions) = parsed_fm.art_generator_instructions {
        fixed_fm.set_art_generator_instructions(art_generator_instructions);
    }
    if let Some(parsed_memes) = parsed_fm.memes {
        fixed_fm.get_memes_mut().extend(parsed_memes);
    }

    println!("--- Exiting Regex-Driven YAML Fixer ---");
    Ok(())
}