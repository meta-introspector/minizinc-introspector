// This file contains the parse_front_matter_fields function.
// It is responsible for parsing basic front matter fields (title, summary, etc.)
// from a YAML string and updating the FixedFrontMatter struct.

use anyhow::{Result, anyhow};
use serde_yaml;
use crate::functions::types::FixedFrontMatter; // Import FixedFrontMatter from types module

#[allow(dead_code)]
pub fn parse_front_matter_fields(
    front_matter_str_for_parsing: &str,
    fixed_fm: &mut FixedFrontMatter,
) -> Result<()> {
    let parsed_front_matter: serde_yaml::Value = serde_yaml::from_str(front_matter_str_for_parsing)
        .map_err(|e| anyhow!("Failed to parse front matter YAML: {}", e))?;

    if let Some(title) = parsed_front_matter.get("title").and_then(|v| v.as_str()) {
        fixed_fm.title = Some(title.to_string());
    }
    if let Some(summary) = parsed_front_matter.get("summary").and_then(|v| v.as_str()) {
        fixed_fm.summary = Some(summary.to_string());
    }
    if let Some(keywords) = parsed_front_matter.get("keywords").and_then(|v| v.as_str()) {
        fixed_fm.keywords = Some(keywords.to_string());
    }
    if let Some(emojis) = parsed_front_matter.get("emojis").and_then(|v| v.as_str()) {
        fixed_fm.emojis = Some(emojis.to_string());
    }
    if let Some(art_generator_instructions) = parsed_front_matter.get("art_generator_instructions").and_then(|v| v.as_str()) {
        fixed_fm.art_generator_instructions = Some(art_generator_instructions.to_string());
    }

    Ok(())
}