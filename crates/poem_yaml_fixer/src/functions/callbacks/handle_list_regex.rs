use anyhow::Result;
//use std::collections::HashMap;
use poem_traits::PoemFrontMatterTrait;
use poem_macros::poem_function;
//use crate::functions::generate_regex::generate_generalized_regex;

#[poem_function(
    name = "list_item",
    pattern = r"^\*\(([^)]*)\)",
    title = "List Item",
    summary = "Extracts a list item.",
    keywords = "list, item",
    emojis = "📝",
    art_generator_instructions = "Generate an image of a bulleted list.",
    pending_meme_description = "This callback extracts a list item."
)]
pub fn handle_list_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<()> {
    fixed_fm.get_raw_meme_lines_mut().push(format!("list: {}", captures[1].trim()));
    Ok(())
}
