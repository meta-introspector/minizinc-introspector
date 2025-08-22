use anyhow::Result;
use crate::functions::types::FixedFrontMatter;
//use poem_traits::{CallbackFn, PoemFunctionMetadata, RegexEntry};
use poem_macros::poem_function; // Import the macro
use poem_traits::PoemFrontMatterTrait;

#[poem_function(
    regex_entry = RegexEntry {
        name: "keywords_comma_separated".to_string(),
        pattern: "keywords: (.*)".to_string(), // This regex will capture the comma-separated string
        callback_function: "handle_comma_separated_keywords".to_string(),
    }
)]

pub fn handle_comma_separated_keywords(
    _line: &str, // The line that matched the regex (not directly used here, but part of signature)
    captures: Vec<String>, // The captured groups from the regex
    fixed_fm_trait: &mut dyn PoemFrontMatterTrait, // Changed type here
) -> Result<()> {
    let fixed_fm = fixed_fm_trait.as_any_mut().downcast_mut::<FixedFrontMatter>().unwrap();
    if let Some(keywords_str) = captures.get(1) { // Assuming the keywords string is the first captured group
        let keywords: Vec<String> = keywords_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        fixed_fm.keywords = Some(keywords);
    }
    Ok(())
}
