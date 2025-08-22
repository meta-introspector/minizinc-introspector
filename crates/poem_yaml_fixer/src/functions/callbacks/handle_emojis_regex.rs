use poem_traits::PoemFrontMatterTrait;

#[poem_macros::poem_function(
    name = "emojis_field",
    pattern = r"emojis:\s*(.*)",
    title = "Poem Emojis",
    summary = "Extracts and sets the emojis for the poem.",
    keywords = "emojis, metadata, symbols",
    emojis = "✨😊",
    art_generator_instructions = "Generate an image of various emojis floating.",
    pending_meme_description = "This is a pending description for the emojis field."
)]
#[allow(dead_code)] // This function is called dynamically via the function registry
pub fn handle_emojis_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> anyhow::Result<()> {
    fixed_fm.set_emojis(captures[1].trim().to_string());
    Ok(())
}
