use anyhow::Result;
use std::collections::HashMap;
use poem_traits::PoemFrontMatterTrait;
use poem_macros::poem_function;
use crate::functions::generate_regex::generate_generalized_regex;

#[poem_function(
    name = "title_raw",
    pattern = r"^\s*-\s*title:\s*(.*)",
    title = "Raw Title (from list item)",
    summary = "Extracts a raw title from a list item.",
    keywords = "title, raw, list",
    emojis = "📝",
    art_generator_instructions = "Generate an image of a title in a list.",
    pending_meme_description = "This callback extracts a raw title from a list item."
)]
pub fn handle_title_raw_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<()> {
    fixed_fm.raw_meme_lines.get_or_insert_with(Vec::new).push(format!("title_raw: {}", captures[1].trim()));
    Ok(())
}