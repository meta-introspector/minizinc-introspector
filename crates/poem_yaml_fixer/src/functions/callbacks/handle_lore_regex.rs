use anyhow::Result;
use std::collections::HashMap;
use poem_traits::PoemFrontMatterTrait;
use poem_macros::poem_function;
use crate::functions::generate_regex::generate_generalized_regex;

#[poem_function(
    name = "lore_field",
    pattern = r#"^\s*lore:\s*"([^"]*)""#,
    title = "Lore Field",
    summary = "Extracts the lore field.",
    keywords = "lore, metadata",
    emojis = "📚",
    art_generator_instructions = "Generate an image of an ancient book.",
    pending_meme_description = "This callback extracts the lore field."
)]
pub fn handle_lore_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<()> {
    fixed_fm.get_raw_meme_lines_mut().push(format!("lore: {}", captures[1].trim()));
    Ok(())
}
