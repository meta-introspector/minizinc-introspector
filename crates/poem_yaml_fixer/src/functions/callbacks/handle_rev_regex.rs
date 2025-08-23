use anyhow::Result;
//use std::collections::HashMap;
use poem_traits::PoemFrontMatterTrait;
use poem_macros::poem_function;
//use crate::functions::generate_regex::generate_generalized_regex;

#[poem_function(
    name = "revision",
    pattern = r"^##\s*Lost Revision from commit\s*(.*)",
    title = "Lost Revision",
    summary = "Extracts lost revision information from a commit.",
    keywords = "revision, commit, lost",
    emojis = "📜",
    art_generator_instructions = "Generate an image of a lost scroll.",
    pending_meme_description = "This callback extracts lost revision information."
)]
pub fn handle_rev_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<()> {
    fixed_fm.get_raw_meme_lines_mut().push(format!("revision: {}", captures[1].trim()));
    Ok(())
}
