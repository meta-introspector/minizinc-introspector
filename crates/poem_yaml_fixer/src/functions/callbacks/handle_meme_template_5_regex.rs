use anyhow::Result;
use std::collections::HashMap;
use poem_traits::PoemFrontMatterTrait;
use poem_macros::poem_function;
use crate::functions::generate_regex::generate_generalized_regex;

#[poem_function(
    name = "meme_template_5",
    pattern = r#"^\s*-\s*"([^"]*)"\s*\(([^)]*)\)"#,
    title = "Meme Template 5",
    summary = "Extracts meme template from pattern 5.",
    keywords = "meme, template",
    emojis = "🖼️",
    art_generator_instructions = "Generate an image of a meme template.",
    pending_meme_description = "This callback extracts meme template from pattern 5."
)]
pub fn handle_meme_template_5_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<()> {
    fixed_fm.get_raw_meme_lines_mut().push(format!(r#"- "{}" ({})"#, captures[1].trim(), captures[2].trim()));
    Ok(())
}
