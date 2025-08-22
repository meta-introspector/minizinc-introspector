use anyhow::{Result, anyhow};
//use crate::functions::types::FixedFrontMatter;
use poem_traits::{CallbackFn, PoemFunctionMetadata, RegexEntry, PoemFrontMatterTrait};
use poem_macros::poem_function;
//use crate::functions::regex_patterns::UNQUOTED_COLON_REGEX_PATTERN; // New import
#[poem_function(
    regex_entry = RegexEntry {
        name: "unquoted_colon_in_meme_description".to_string(),
        pattern: UNQUOTED_COLON_REGEX_PATTERN.to_string(), // Use constant
        callback_function: "handle_unquoted_colon_in_description".to_string(),
    }
)]
pub fn handle_unquoted_colon_in_description(
    _line: &str,
    captures: Vec<String>,
    _fixed_fm_trait: &mut dyn PoemFrontMatterTrait, // Changed type here
) -> Result<()> {
    let problematic_description = captures.get(1).map_or("".to_string(), |s| s.clone());
    Err(anyhow!(
        "YAML parsing error: Unquoted colon in meme description. Problematic content: \"{}.\" Requires manual fix or advanced YAML parsing logic.",
        problematic_description
    ))
}



