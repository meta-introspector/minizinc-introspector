use anyhow::Result;
//use std::collections::HashMap;
use poem_traits::PoemFrontMatterTrait;
use poem_macros::poem_function;
//use crate::functions::generate_regex::generate_generalized_regex;

#[poem_function(
    name = "filename_index_emoji",
    pattern = r"^Filename:\s*(.*)",
    title = "Filename Index Emoji",
    summary = "Extracts filename and associated emojis.",
    keywords = "filename, index, emoji",
    emojis = "📄",
    art_generator_instructions = "Generate an image of a file with a name tag.",
    pending_meme_description = "This callback extracts filename and associated emojis."
)]
pub fn handle_filename_index_emoji_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<()> {
    fixed_fm.get_raw_meme_lines_mut().push(format!("filename_index_emoji: {}", captures[1].trim()));
    Ok(())
}
