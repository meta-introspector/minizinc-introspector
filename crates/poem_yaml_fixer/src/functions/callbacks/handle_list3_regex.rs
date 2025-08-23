use anyhow::Result;
use std::collections::HashMap;
use poem_traits::PoemFrontMatterTrait;
use poem_macros::poem_function;
use crate::functions::generate_regex::generate_generalized_regex;

#[poem_function(
    name = "list_item_3",
    pattern = r"^-\s*(.*)",
    title = "List Item 3",
    summary = "Extracts a generic list item.",
    keywords = "list, item",
    emojis = "📝",
    art_generator_instructions = "Generate an image of a bulleted list.",
    pending_meme_description = "This callback extracts a generic list item."
)]
pub fn handle_list3_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<()> {
    fixed_fm.get_raw_meme_lines_mut().push(format!("list3: {}", captures[1].trim()));
    Ok(())
}
