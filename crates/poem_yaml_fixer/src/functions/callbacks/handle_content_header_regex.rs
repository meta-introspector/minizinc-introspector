use anyhow::Result;
use std::collections::HashMap;
use poem_traits::PoemFrontMatterTrait;
use poem_macros::poem_function;
use crate::functions::generate_regex::generate_generalized_regex;

#[poem_function(
    name = "content_header",
    pattern = r"^\s*#\s*(.*)",
    title = "Content Header",
    summary = "Extracts a content header (H1).",
    keywords = "header, content, H1",
    emojis = "📑",
    art_generator_instructions = "Generate an image of a large, bold title.",
    pending_meme_description = "This callback extracts an H1 content header."
)]
pub fn handle_content_header_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<()> {
    fixed_fm.raw_meme_lines.get_or_insert_with(Vec::new).push(format!("content_header: {}", captures[1].trim()));
    Ok(())
}