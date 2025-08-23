use anyhow::Result;
use std::collections::HashMap;
use poem_traits::PoemFrontMatterTrait;
use poem_macros::poem_function;
use crate::functions::generate_regex::generate_generalized_regex;

#[poem_function(
    name = "description_raw",
    pattern = r"^\s*\(([^)]*)\)",
    title = "Raw Description",
    summary = "Extracts a raw description from parentheses.",
    keywords = "description, raw, parentheses",
    emojis = "📝",
    art_generator_instructions = "Generate an image of text in parentheses.",
    pending_meme_description = "This callback extracts a raw description from parentheses."
)]
pub fn handle_description_raw_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<()> {
    fixed_fm.raw_meme_lines.get_or_insert_with(Vec::new).push(format!("description_raw: {}", captures[1].trim()));
    Ok(())
}