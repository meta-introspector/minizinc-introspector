use anyhow::Result;
use std::collections::HashMap;
use poem_traits::PoemFrontMatterTrait;
use poem_macros::poem_function;
use crate::functions::generate_regex::generate_generalized_regex;

#[poem_function(
    name = "multi_line_description",
    pattern = r"^\s*description:\s*\|",
    title = "Multi-line Description Start",
    summary = "Indicates the start of a multi-line description.",
    keywords = "description, multi-line, metadata",
    emojis = "📝",
    art_generator_instructions = "Generate an image of a scroll with flowing text.",
    pending_meme_description = "This callback indicates the start of a multi-line description."
)]
pub fn handle_desc_regex(_line: &str, _captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<()> {
    fixed_fm.set_summary(String::new()); // Initialize summary for multi-line content
    Ok(())
}