use anyhow::Result;
use std::collections::HashMap;
use poem_traits::PoemFrontMatterTrait;
use poem_macros::poem_function;
use crate::functions::generate_regex::generate_generalized_regex;

#[poem_function(
    name = "meme_template_3",
    pattern = r"^template:\s*(.*)",
    title = "Meme Template 3",
    summary = "Extracts meme template from pattern 3.",
    keywords = "meme, template",
    emojis = "🖼️",
    art_generator_instructions = "Generate an image of a meme template.",
    pending_meme_description = "This callback extracts meme template from pattern 3."
)]
pub fn handle_meme_template_3_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<()> {
    fixed_fm.get_raw_meme_lines_mut().push(format!("template: {}", captures[1].trim()));
    Ok(())
}
