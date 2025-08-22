use anyhow::Result;
use poem_traits::PoemFrontMatterTrait;

#[poem_macros::poem_function(
    name = "new_meme_description",
    pattern = r"desc:\s*(.*)",
    title = "New Meme Description Handler",
    summary = "Handles parsing of new meme description format.",
    keywords = "meme, new_format, description",
    emojis = "🆕📝",
    art_generator_instructions = "Generate an image of a fresh notepad with a pen.",
    pending_meme_description = "This is a pending description for a new meme description."
)]

pub fn handle_new_meme_desc_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<(), anyhow::Error> {
    *fixed_fm.get_pending_meme_description_mut() = Some(captures[1].trim().to_string());
    Ok(())
}
