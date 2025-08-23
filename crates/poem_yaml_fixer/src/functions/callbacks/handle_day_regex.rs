use anyhow::Result;
//use std::collections::HashMap;
use poem_traits::PoemFrontMatterTrait;
use poem_macros::poem_function;
//use crate::functions::generate_regex::generate_generalized_regex;

#[poem_function(
    name = "day_field",
    pattern = r"^\s*day:\s*(.*)",
    title = "Day Field",
    summary = "Extracts the day field.",
    keywords = "day, metadata",
    emojis = "📅",
    art_generator_instructions = "Generate an image of a calendar.",
    pending_meme_description = "This callback extracts the day field."
)]
pub fn handle_day_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<()> {
    fixed_fm.get_raw_meme_lines_mut().push(format!("day: {}", captures[1].trim()));
    Ok(())
}
