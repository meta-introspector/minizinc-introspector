use anyhow::Result;
//use std::collections::HashMap;
use poem_traits::PoemFrontMatterTrait;
use poem_macros::poem_function;
//use crate::functions::generate_regex::generate_generalized_regex;

#[poem_function(
    name = "art_instructions",
    pattern = r"^\s*art_generator_instructions:\s*(.*)",
    title = "Art Generator Instructions",
    summary = "Extracts instructions for art generation.",
    keywords = "art, generator, instructions",
    emojis = "🎨",
    art_generator_instructions = "Generate an image of a painter's palette with a brush.",
    pending_meme_description = "This callback extracts art generator instructions."
)]
pub fn handle_art_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<()> {
    fixed_fm.set_art_generator_instructions(captures[1].trim().to_string());
    Ok(())
}
