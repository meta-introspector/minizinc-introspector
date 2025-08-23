use anyhow::Result;
use std::collections::HashMap;
use poem_traits::PoemFrontMatterTrait;
use poem_macros::poem_function;
use crate::functions::generate_regex::generate_generalized_regex;

#[poem_function(
    name = "quote_content",
    pattern = r#"^"([^"]*)""#,
    title = "Quote Content",
    summary = "Appends quoted content to the poem body.",
    keywords = "content, body, quote",
    emojis = "📜",
    art_generator_instructions = "Generate an image of quoted text.",
    pending_meme_description = "This callback appends quoted content to the poem body."
)]
pub fn handle_quote_content_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<()> {
    fixed_fm.poem_body.get_or_insert_with(String::new).push_str(&format!("{}\n", captures[1].trim()));
    Ok(())
}
