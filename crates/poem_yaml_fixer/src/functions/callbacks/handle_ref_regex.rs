use anyhow::Result;
use std::collections::HashMap;
use poem_traits::PoemFrontMatterTrait;
use poem_macros::poem_function;
use crate::functions::generate_regex::generate_generalized_regex;

#[poem_function(
    name = "reference",
    pattern = r"^\*\s*\[([^\]]*)\]\(([^)]*)\)",
    title = "Reference",
    summary = "Extracts a reference link.",
    keywords = "reference, link",
    emojis = "🔗",
    art_generator_instructions = "Generate an image of a chain link.",
    pending_meme_description = "This callback extracts a reference link."
)]
pub fn handle_ref_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<()> {
    fixed_fm.raw_meme_lines.get_or_insert_with(Vec::new).push(format!("ref: {} ({})", captures[1].trim(), captures[2].trim()));
    Ok(())
}