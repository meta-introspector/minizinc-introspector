use anyhow::Result;
use std::collections::HashMap;
use poem_traits::PoemFrontMatterTrait;
use poem_macros::poem_function;
use crate::functions::generate_regex::generate_generalized_regex;

#[poem_function(
    name = "numerology_field",
    pattern = r#"^\s*numerology:\s*"([^"]*)""#,
    title = "Numerology Field",
    summary = "Extracts the numerology field.",
    keywords = "numerology, metadata",
    emojis = "🔢",
    art_generator_instructions = "Generate an image of numbers and symbols.",
    pending_meme_description = "This callback extracts the numerology field."
)]
pub fn handle_numerology_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<()> {
    fixed_fm.get_raw_meme_lines_mut().push(format!("numerology: {}", captures[1].trim()));
    Ok(())
}
