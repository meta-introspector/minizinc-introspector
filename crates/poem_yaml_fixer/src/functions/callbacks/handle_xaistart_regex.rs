use anyhow::Result;
//use std::collections::HashMap;
use poem_traits::PoemFrontMatterTrait;
use poem_macros::poem_function;
//use crate::functions::generate_regex::generate_generalized_regex;

#[poem_function(
    name = "xai_start",
    pattern = r#"^<xaiArtifact\s+artifact_id="([^"]*)"\s+artifact_version_id="([^"]*)"\s+title="([^"]*)"\s+contentType="([^"]*)">"#,
    title = "XAI Artifact Start Tag",
    summary = "Matches the start tag for an XAI artifact and extracts its attributes.",
    keywords = "xai, artifact, start tag, attributes",
    emojis = "🚀",
    art_generator_instructions = "Generate an image of an opening tag with data flowing out.",
    pending_meme_description = "This callback matches the start tag for an XAI artifact and extracts its attributes."
)]
pub fn handle_xaistart_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<()> {
    fixed_fm.get_raw_meme_lines_mut().push(format!("xaistart: id={}, version={}, title={}, type={}", captures[1].trim(), captures[2].trim(), captures[3].trim(), captures[4].trim()));
    Ok(())
}
