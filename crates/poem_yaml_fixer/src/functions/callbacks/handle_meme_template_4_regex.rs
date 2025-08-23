use anyhow::Result;
use std::collections::HashMap;
use poem_traits::PoemFrontMatterTrait;
use poem_macros::poem_function;
use crate::functions::generate_regex::generate_generalized_regex;

#[poem_function(
    name = "meme_template_4",
    pattern = r#"^\s*MemeTemplate:\s*"([^"]*)""#,
    title = "Meme Template 4",
    summary = "Extracts meme template from pattern 4.",
    keywords = "meme, template",
    emojis = "🖼️",
    art_generator_instructions = "Generate an image of a meme template.",
    pending_meme_description = "This callback extracts meme template from pattern 4."
)]
pub fn handle_meme_template_4_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<()> {
    fixed_fm.raw_meme_lines.get_or_insert_with(Vec::new).push(format!("MemeTemplate: {}", captures[1].trim()));
    Ok(())
}