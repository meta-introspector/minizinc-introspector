use poem_traits::PoemFrontMatterTrait;

#[poem_macros::poem_function(
    name = "title_field",
    pattern = r"title:\s*(.*)",
    title = "Poem Title",
    summary = "Extracts and sets the title of the poem.",
    keywords = "title, metadata",
    emojis = "📝👑",
    art_generator_instructions = "Generate an image of a regal title scroll.",
    pending_meme_description = "This is a pending description for the title field."
)]

pub fn handle_title_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> anyhow::Result<()> {
    fixed_fm.set_title(captures[1].trim().to_string());
    Ok(())
}
