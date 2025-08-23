use anyhow::Result;
//use std::collections::HashMap;
use poem_traits::PoemFrontMatterTrait;
use poem_macros::poem_function;
//use crate::functions::generate_regex::generate_generalized_regex;

#[poem_function(
    name = "content_header_2",
    pattern = r"^\s*##\s*(.*)",
    title = "Content Header 2",
    summary = "Extracts a second-level header (H2).",
    keywords = "header, content, H2",
    emojis = "📑",
    art_generator_instructions = "Generate an image of a medium-sized title.",
    pending_meme_description = "This callback extracts an H2 content header."
)]
pub fn handle_content_header_2_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<()> {
    fixed_fm.get_raw_meme_lines_mut().push(format!("content_header2: {}", captures[1].trim()));
    Ok(())
}
