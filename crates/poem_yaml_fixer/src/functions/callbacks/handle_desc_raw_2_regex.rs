use anyhow::Result;
use std::collections::HashMap;
use poem_traits::PoemFrontMatterTrait;
use poem_macros::poem_function;
use crate::functions::generate_regex::generate_generalized_regex;

#[poem_function(
    name = "desc_raw_2",
    pattern = r#"^-\s*description:\s*'([^']*)'"#,
    title = "Raw Description 2 (from list item)",
    summary = "Extracts a raw description from a list item with single quotes.",
    keywords = "description, raw, list, single quotes",
    emojis = "📝",
    art_generator_instructions = "Generate an image of a description in a list with single quotes.",
    pending_meme_description = "This callback extracts a raw description from a list item with single quotes."
)]
pub fn handle_desc_raw_2_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<()> {
    fixed_fm.get_raw_meme_lines_mut().push(format!("desc_raw_2: {}", captures[1].trim()));
    Ok(())
}