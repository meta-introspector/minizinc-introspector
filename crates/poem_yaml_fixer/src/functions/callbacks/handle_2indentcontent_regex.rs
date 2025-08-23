use anyhow::Result;
use std::collections::HashMap;
use poem_traits::PoemFrontMatterTrait;
use poem_macros::poem_function;
use crate::functions::generate_regex::generate_generalized_regex;

#[poem_function(
    name = "two_indent_content",
    pattern = r"^\s{2,}.*",
    title = "Two Indent Content",
    summary = "Appends content with two or more indents to the poem body.",
    keywords = "content, body, indent",
    emojis = "📜",
    art_generator_instructions = "Generate an image of indented text.",
    pending_meme_description = "This callback appends content with two or more indents to the poem body."
)]
pub fn handle_2indentcontent_regex(_line: &str, _captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<() {
    fixed_fm.poem_body.get_or_insert_with(String::new).push_str(&format!("{}\n", _line));
    Ok(())
}