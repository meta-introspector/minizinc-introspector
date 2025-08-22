use anyhow::Result;
use poem_traits::{PoemFrontMatterTrait, Meme};

#[poem_macros::poem_function(
    name = "new_meme_template",
    pattern = r"template:\s*(.*)",
    title = "New Meme Template Handler",
    summary = "Handles parsing of new meme template format.",
    keywords = "meme, new_format, template",
    emojis = "🆕🖼️",
    art_generator_instructions = "Generate an image of a blank canvas with a brush.",
    pending_meme_description = "This is a pending description for a new meme template."
)]
#[allow(dead_code)] // This function is called dynamically via the function registry
pub fn handle_new_meme_template_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<(), anyhow::Error> {
    if let Some(description) = fixed_fm.get_pending_meme_description_mut().take() {
        let template = captures[1].trim().to_string();
        fixed_fm.get_memes_mut().push(Meme {
            description,
            template,
            traits: None,
            nft_id: None,
            lore: None,
            numerology: None,
        });
    } else {
        eprintln!("Warning: Template found without a preceding description for new meme format.");
    }
    Ok(())
}
