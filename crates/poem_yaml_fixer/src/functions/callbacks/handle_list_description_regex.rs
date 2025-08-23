use anyhow::Result;
//use std::collections::HashMap;
use poem_traits::PoemFrontMatterTrait;
use poem_macros::poem_function;
//use crate::functions::generate_regex::generate_generalized_regex;

#[poem_function(
    name = "list_description_field",
    pattern = r#"^\s*-\s*description:\s*"([^"]*)""#,
    title = "Poem List Description",
    summary = "Extracts and sets the summary of the poem from a list item description.",
    keywords = "summary, description, list, metadata",
    emojis = "📝",
    art_generator_instructions = "Generate an image of a concise summary within a list.",
    pending_meme_description = "This is a pending description for the list description field."
)]
pub fn handle_list_description_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<()> {
    fixed_fm.set_summary(captures[1].trim().to_string());
    Ok(())
}
