use anyhow::Result;
use std::collections::HashMap;
use poem_traits::PoemFrontMatterTrait;
use poem_macros::poem_function;
use crate::functions::generate_regex::generate_generalized_regex;

#[poem_function(
    name = "summary_from_pattern",
    pattern = r#"^\s*description:\s*"([^"]*)""#,
    title = "Poem Summary from Pattern",
    summary = "Extracts and sets the summary of the poem from a description line in patterns.txt.",
    keywords = "summary, description, metadata, pattern",
    emojis = "📝",
    art_generator_instructions = "Generate an image of a concise summary.",
    pending_meme_description = "This is a pending description for the summary field from patterns.txt."
)]
pub fn handle_summary_from_pattern_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<()> {
    fixed_fm.set_summary(captures[1].trim().to_string());
    Ok(())
}