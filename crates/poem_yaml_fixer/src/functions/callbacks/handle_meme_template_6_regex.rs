use anyhow::Result;
//use std::collections::HashMap;
use poem_traits::PoemFrontMatterTrait;
use poem_macros::poem_function;
///use crate::functions::generate_regex::generate_generalized_regex;

#[poem_function(
    name = "meme_template_6",
    pattern = ".",
    title = "Meme Template 6",
    summary = "Extracts meme template from pattern 6.",
    keywords = "meme, template",
    emojis = "🖼️",
    art_generator_instructions = "Generate an image of a meme template.",
    pending_meme_description = "This callback extracts meme template from pattern 6."
)]
pub fn handle_meme_template_6_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<()> {
    fixed_fm.get_raw_meme_lines_mut().push(format!("\"{}\"", captures[1].trim()));
    Ok(())
}
