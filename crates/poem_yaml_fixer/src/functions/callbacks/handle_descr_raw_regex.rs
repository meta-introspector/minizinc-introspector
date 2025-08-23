use anyhow::Result;
use std::collections::HashMap;
use poem_traits::PoemFrontMatterTrait;
use poem_macros::poem_function;
use crate::functions::generate_regex::generate_generalized_regex;

#[poem_function(
    name = "descr_raw",
    pattern = r#"^\s*-\s*description:\s*"([^"]*)""#,
    title = "Raw Description (from list item)",
    summary = "Extracts a raw description from a list item.",
    keywords = "description, raw, list",
    emojis = "📝",
    art_generator_instructions = "Generate an image of a description in a list.",
    pending_meme_description = "This callback extracts a raw description from a list item."
)]
pub fn handle_descr_raw_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<()> {
    fixed_fm.raw_meme_lines.get_or_insert_with(Vec::new).push(format!("descr_raw: {}", captures[1].trim()));
    Ok(())
}