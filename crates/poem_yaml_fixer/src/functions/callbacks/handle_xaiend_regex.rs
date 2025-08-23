use anyhow::Result;
use std::collections::HashMap;
use poem_traits::PoemFrontMatterTrait;
use poem_macros::poem_function;
use crate::functions::generate_regex::generate_generalized_regex;

#[poem_function(
    name = "xai_end",
    pattern = r"^<\/xaiArtifact>",
    title = "XAI Artifact End Tag",
    summary = "Matches the end tag for an XAI artifact.",
    keywords = "xai, artifact, end tag",
    emojis = "🔚",
    art_generator_instructions = "Generate an image of a closing tag.",
    pending_meme_description = "This callback matches the end tag for an XAI artifact."
)]
pub fn handle_xaiend_regex(_line: &str, _captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<()> {
    fixed_fm.get_raw_meme_lines_mut().push("xaiend".to_string());
    Ok(())
}
