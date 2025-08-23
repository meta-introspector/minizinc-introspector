use anyhow::Result;
use std::collections::HashMap;
use poem_traits::PoemFrontMatterTrait;
use poem_macros::poem_function;
use crate::functions::generate_regex::generate_generalized_regex;

#[poem_function(
    name = "header1",
    pattern = r"^#\s*\*\*(.*)\*\*",
    title = "Header 1",
    summary = "Extracts a top-level header (H1) from the content.",
    keywords = "header, content, H1",
    emojis = "📑",
    art_generator_instructions = "Generate an image of a large, bold title.",
    pending_meme_description = "This callback extracts an H1 header."
)]
pub fn handle_header1_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<()> {
    fixed_fm.get_raw_meme_lines_mut().push(format!("header1: {}", captures[1].trim()));
    Ok(())
}
