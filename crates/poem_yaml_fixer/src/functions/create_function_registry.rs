use anyhow::Result;
use linkme::distributed_slice;
use poem_traits::{PoemFrontMatterTrait, Meme}; // Import Meme from poem_traits

// Define the distributed slice where functions will register themselves.
// This static is populated by functions annotated with #[poem_macros::poem_function]
// and is used by the poem_header! macro to create the function registry.
#[distributed_slice]
pub static FUNCTIONS2: [&'static (String, fn() -> Box<dyn Fn(&str, Vec<String>, &mut dyn PoemFrontMatterTrait) -> Result<(), anyhow::Error> + Send + Sync + 'static>)];

// Extracted callback functions with #[poem_function] attribute
#[poem_macros::poem_function]
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

#[poem_macros::poem_function]
pub fn handle_new_meme_desc_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<(), anyhow::Error> {
    *fixed_fm.get_pending_meme_description_mut() = Some(captures[1].trim().to_string());
    Ok(())
}

#[poem_macros::poem_function]
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

#[poem_macros::poem_function]
pub fn handle_title_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> anyhow::Result<()> {
    fixed_fm.set_title(captures[1].trim().to_string());
    Ok(())
}

#[poem_macros::poem_function]
pub fn handle_summary_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> anyhow::Result<()> {
    fixed_fm.set_summary(captures[1].trim().to_string());
    Ok(())
}

#[poem_macros::poem_function]
pub fn handle_keywords_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> anyhow::Result<()> {
    fixed_fm.set_keywords(captures[1].trim().to_string());
    Ok(())
}

#[poem_macros::poem_function]
pub fn handle_emojis_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> anyhow::Result<()> {
    fixed_fm.set_emojis(captures[1].trim().to_string());
    Ok(())
}

#[poem_macros::poem_function]
pub fn handle_art_generator_instructions_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> anyhow::Result<()> {
    fixed_fm.set_art_generator_instructions(captures[1].trim().to_string());
    Ok(())
}

#[poem_macros::poem_function]
pub fn handle_poem_body_start_regex(_line: &str, _captures: Vec<String>, _fixed_fm: &mut dyn PoemFrontMatterTrait) -> anyhow::Result<()> {
    // This function just marks the start of the poem body, no direct action needed here
    Ok(())
}
