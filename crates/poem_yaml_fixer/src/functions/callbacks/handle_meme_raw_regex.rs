use anyhow::Result;
//use std::collections::HashMap;
use poem_traits::PoemFrontMatterTrait;
use poem_macros::poem_function;
//use crate::functions::generate_regex::generate_generalized_regex;

#[poem_function(
    name = "meme_raw",
    pattern = r#"^\s*-\s*"([^\"]*)"\s*\(([^)]*)""#,
    title = "Raw Meme Entry",
    summary = "Extracts a raw meme entry.",
    keywords = "meme, raw",
    emojis = "😂",
    art_generator_instructions = "Generate an image of a raw meme.",
    pending_meme_description = "This callback extracts a raw meme entry."
)]
pub fn handle_meme_raw_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<()> {
    fixed_fm.get_raw_meme_lines_mut().push(format!("meme_raw: \"{}\" ({})", captures[1].trim(), captures[2].trim()));
    Ok(())
}
