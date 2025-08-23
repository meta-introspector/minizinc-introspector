use anyhow::Result;
//use std::collections::HashMap;
use poem_traits::PoemFrontMatterTrait;
use poem_macros::poem_function;
//use crate::functions::generate_regex::generate_generalized_regex;

#[poem_function(
    name = "concept_field",
    pattern = r#"^\s*-\s*Concept:\s*"([^"]*)""#,
    title = "Concept Field",
    summary = "Extracts a concept field.",
    keywords = "concept, metadata",
    emojis = "💡",
    art_generator_instructions = "Generate an image of a lightbulb with ideas.",
    pending_meme_description = "This callback extracts a concept field."
)]
pub fn handle_concept_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<()> {
    fixed_fm.get_raw_meme_lines_mut().push(format!("concept: {}", captures[1].trim()));
    Ok(())
}
