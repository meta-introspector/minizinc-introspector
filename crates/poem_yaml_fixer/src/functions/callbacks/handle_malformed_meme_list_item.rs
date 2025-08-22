use anyhow::Result;
use crate::functions::types::FixedFrontMatter;
use poem_traits::{CallbackFn, PoemFunctionMetadata, RegexEntry, Meme};
use poem_macros::poem_function;

#[poem_function(
    regex_entry = RegexEntry {
        name: "malformed_meme_list_item".to_string(),
        pattern: r#"^- "([^"]+)" \(([^)]+)\)"#.to_string(), // Regex to capture description and template in parentheses
        callback_function: "handle_malformed_meme_list_item".to_string(),
    }
)]
pub fn handle_malformed_meme_list_item(
    _line: &str,
    captures: Vec<String>,
    fixed_fm_trait: &mut dyn PoemFrontMatterTrait,
) -> Result<()> {
    let fixed_fm = fixed_fm_trait.as_any_mut().downcast_mut::<FixedFrontMatter>().unwrap();
    let description = captures.get(1).map_or("".to_string(), |s| s.clone());
    let template_raw = captures.get(2).map_or("".to_string(), |s| s.clone());

    // Extract template name from "Template Name meme" or "Template Name meme, with extra text"
    let template = template_raw
        .to_lowercase()
        .replace(" meme", "")
        .split(',')
        .next()
        .map_or("unknown".to_string(), |s| s.trim().to_string());

    let new_meme = Meme {
        description: description,
        template: template,
        traits: None, // Cannot extract from this format
        nft_id: None, // Cannot extract from this format
        lore: None, // Cannot extract from this format
        numerology: None, // Cannot extract from this format
    };

    if let Some(memes) = &mut fixed_fm.memes {
        memes.push(new_meme);
    } else {
        fixed_fm.memes = Some(vec![new_meme]);
    }

    Ok(())
}
