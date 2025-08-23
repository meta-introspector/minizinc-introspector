use anyhow::Result;
//use std::collections::HashMap;
use poem_traits::PoemFrontMatterTrait;
use poem_macros::poem_function;
//use crate::functions::generate_regex::generate_generalized_regex;

#[poem_function(
    name = "nft_id_field",
    pattern = r#"^\s*nft_id:\s*"([^"]*)""#,
    title = "NFT ID Field",
    summary = "Extracts the NFT ID field.",
    keywords = "nft, id, metadata",
    emojis = "🆔",
    art_generator_instructions = "Generate an image of a digital ID card.",
    pending_meme_description = "This callback extracts the NFT ID field."
)]
pub fn handle_nft_id_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<()> {
    fixed_fm.get_raw_meme_lines_mut().push(format!("nft_id: {}", captures[1].trim()));
    Ok(())
}
