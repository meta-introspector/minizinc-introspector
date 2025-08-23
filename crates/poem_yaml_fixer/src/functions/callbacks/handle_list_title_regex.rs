use anyhow::Result;
//use std::collections::HashMap;
use poem_traits::PoemFrontMatterTrait;
use poem_macros::poem_function;
//use crate::functions::generate_regex::generate_generalized_regex;

#[poem_function(
    name = "list_title_field",
    pattern = r#"^\s*-\s*title:\s*(.*)"#,
    title = "Poem List Title",
    summary = "Extracts and sets the title of the poem from a list item.",
    keywords = "title, list, metadata",
    emojis = "📝👑",
    art_generator_instructions = "Generate an image of a regal title scroll within a list.",
    pending_meme_description = "This is a pending description for the list title field."
)]
pub fn handle_list_title_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<()> {
    fixed_fm.set_title(captures[1].trim().to_string());
    Ok(())
}
