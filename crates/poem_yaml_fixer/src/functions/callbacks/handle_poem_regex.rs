use anyhow::Result;
//use std::collections::HashMap;
use poem_traits::PoemFrontMatterTrait;
use poem_macros::poem_function;
//use crate::functions::generate_regex::generate_generalized_regex;

#[poem_function(
    name = "poem_body_start",
    pattern = r"^Poem:\s*\|",
    title = "Poem Body Start",
    summary = "Initializes the poem body.",
    keywords = "poem, body, start",
    emojis = "📜",
    art_generator_instructions = "Generate an image of a scroll unfurling.",
    pending_meme_description = "This callback initializes the poem body."
)]
pub fn handle_poem_regex(_line: &str, _captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<()> {
    fixed_fm.set_poem_body(String::new());
    Ok(())
}
