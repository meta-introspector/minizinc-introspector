use poem_traits::PoemFrontMatterTrait;

#[poem_macros::poem_function(
    name = "keywords_field",
    pattern = r"keywords:\s*(.*)",
    title = "Poem Keywords",
    summary = "Extracts and sets the keywords for the poem.",
    keywords = "keywords, metadata, tags",
    emojis = "🏷️🔑",
    art_generator_instructions = "Generate an image of a tag cloud.",
    pending_meme_description = "This is a pending description for the keywords field."
)]
#[allow(dead_code)] // This function is called dynamically via the function registry
pub fn handle_keywords_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> anyhow::Result<()> {
    fixed_fm.set_keywords(captures[1].trim().to_string());
    Ok(())
}
