use anyhow::Result;
use std::collections::HashMap;
use poem_traits::PoemFrontMatterTrait;
use poem_macros::poem_function;
use crate::functions::generate_regex::generate_generalized_regex;

#[poem_function(
    name = "indented_content",
    pattern = r"^\s{6,}.*",
    title = "Indented Content",
    summary = "Appends indented content to the poem body.",
    keywords = "content, body, indent",
    emojis = "📜",
    art_generator_instructions = "Generate an image of indented text.",
    pending_meme_description = "This callback appends indented content to the poem body."
)]
pub fn handle_indented_content_regex(_line: &str, _captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<()> {
    fixed_fm.get_poem_body_mut().push_str(&format!("{}\n", _line));
    Ok(())
}
