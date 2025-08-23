use anyhow::Result;
use std::collections::HashMap;
use poem_traits::PoemFrontMatterTrait;
use poem_macros::poem_function;
use crate::functions::generate_regex::generate_generalized_regex;

#[poem_function(
    name = "traits_field",
    pattern = r"^\s*traits:\s*\[([^\]]*)\]",
    title = "Traits Field",
    summary = "Extracts the traits field.",
    keywords = "traits, metadata",
    emojis = "✨",
    art_generator_instructions = "Generate an image of abstract qualities.",
    pending_meme_description = "This callback extracts the traits field."
)]
pub fn handle_traits_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<()> {
    fixed_fm.raw_meme_lines.get_or_insert_with(Vec::new).push(format!("traits: {}", captures[1].trim()));
    Ok(())
}
