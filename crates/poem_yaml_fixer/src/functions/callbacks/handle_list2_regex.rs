use anyhow::Result;
//use std::collections::HashMap;
use poem_traits::PoemFrontMatterTrait;
use poem_macros::poem_function;
//use crate::functions::generate_regex::generate_generalized_regex;

#[poem_function(
    name = "list_item_2",
    pattern = r"^-\s*\*\*(.*?)\*\*:\s*(.*)",
    title = "List Item 2",
    summary = "Extracts a list item with bold text.",
    keywords = "list, item, bold",
    emojis = "📝",
    art_generator_instructions = "Generate an image of a bulleted list with bold text.",
    pending_meme_description = "This callback extracts a list item with bold text."
)]
pub fn handle_list2_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<()> {
    fixed_fm.get_raw_meme_lines_mut().push(format!("list2: {}: {}", captures[1].trim(), captures[2].trim()));
    Ok(())
}
