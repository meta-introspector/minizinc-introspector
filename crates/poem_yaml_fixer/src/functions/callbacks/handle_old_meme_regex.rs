use anyhow::Result;
use poem_traits::{PoemFrontMatterTrait, Meme};

#[poem_macros::poem_function(
    name = "old_meme_format",
    pattern = r"description:\s*(.*)\s*template:\s*(.*)",
    title = "Old Meme Format Handler",
    summary = "Handles parsing of old meme format with description and template.",
    keywords = "meme, old_format, description, template",
    emojis = "👴📜",
    art_generator_instructions = "Generate an image representing an old scroll with faded text.",
    pending_meme_description = "This is a pending description for an old meme."
)]

pub fn handle_old_meme_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<(), anyhow::Error> {
    let description = captures[1].trim().to_string();
    let template = captures[2].trim().to_string();
    fixed_fm.get_memes_mut().push(Meme {
        description,
        template,
        traits: None,
        nft_id: None,
        lore: None,
        numerology: None,
    });
    Ok(())
}
