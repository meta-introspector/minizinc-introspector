use anyhow::Result;
//use std::collections::HashMap;
use poem_traits::PoemFrontMatterTrait;
use poem_macros::poem_function;
//use crate::functions::generate_regex::generate_generalized_regex;

#[poem_function(
    name = "named_file",
    pattern = r"^###\s*(.*)\s*\(([^)]*)\)",
    title = "Named File",
    summary = "Extracts named file information.",
    keywords = "file, name, metadata",
    emojis = "📄",
    art_generator_instructions = "Generate an image of a file with a name tag.",
    pending_meme_description = "This callback extracts named file information."
)]
pub fn handle_named_file_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<()> {
    fixed_fm.get_raw_meme_lines_mut().push(format!("named_file: {} ({})", captures[1].trim(), captures[2].trim()));
    Ok(())
}
