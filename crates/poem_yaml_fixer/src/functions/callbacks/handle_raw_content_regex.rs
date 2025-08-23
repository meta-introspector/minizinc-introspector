use anyhow::Result;
use std::collections::HashMap;
use poem_traits::PoemFrontMatterTrait;
use poem_macros::poem_function;
use crate::functions::generate_regex::generate_generalized_regex;

#[poem_function(
    name = "raw_content",
    pattern = r"^(.*)",
    title = "Raw Content",
    summary = "Appends any raw content line to the poem body.",
    keywords = "raw, content, body",
    emojis = "📜",
    art_generator_instructions = "Generate an image of raw text.",
    pending_meme_description = "This callback appends any raw content line to the poem body."
)]
pub fn handle_raw_content_regex(_line: &str, _captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<()>
{
    fixed_fm.get_poem_body_mut().push_str(&format!("{}\n", _line));
    Ok(())
}
