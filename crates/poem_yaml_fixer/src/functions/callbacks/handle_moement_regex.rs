use anyhow::Result;
//use std::collections::HashMap;
use poem_traits::PoemFrontMatterTrait;
use poem_macros::poem_function;
//use crate::functions::generate_regex::generate_generalized_regex;

#[poem_function(
    name = "movement_field",
    pattern = r#"^\s*-\s*movement:\s*"([^"]*)""#,
    title = "Movement Field",
    summary = "Extracts a movement field.",
    keywords = "movement, metadata",
    emojis = "🎶",
    art_generator_instructions = "Generate an image of musical notes in motion.",
    pending_meme_description = "This callback extracts a movement field."
)]
pub fn handle_moement_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<()> {
    fixed_fm.get_raw_meme_lines_mut().push(format!("movement: {}", captures[1].trim()));
    Ok(())
}
