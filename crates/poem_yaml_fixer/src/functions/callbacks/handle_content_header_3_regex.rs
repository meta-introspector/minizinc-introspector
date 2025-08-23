use anyhow::Result;
use std::collections::HashMap;
use poem_traits::PoemFrontMatterTrait;
use poem_macros::poem_function;
use crate::functions::generate_regex::generate_generalized_regex;

#[poem_function(
    name = "content_header_3",
    pattern = r"^\s*###\s*(.*)",
    title = "Content Header 3",
    summary = "Extracts a content header (H3).",
    keywords = "header, content, H3",
    emojis = "📑",
    art_generator_instructions = "Generate an image of a small title.",
    pending_meme_description = "This callback extracts an H3 content header."
)]
pub fn handle_content_header_3_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<()> {
    fixed_fm.get_raw_meme_lines_mut().push(format!("content_header3: {}", captures[1].trim()));
    Ok(())
}
