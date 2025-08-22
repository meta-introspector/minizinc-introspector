use anyhow::Result;
use poem_traits::PoemFrontMatterTrait;

#[poem_macros::poem_function(
    name = "summary_field",
    pattern = r"summary:\s*(.*)",
    title = "Poem Summary",
    summary = "Extracts and sets the summary of the poem.",
    keywords = "summary, metadata",
    emojis = "📝📄",
    art_generator_instructions = "Generate an image of a concise document.",
    pending_meme_description = "This is a pending description for the summary field."
)]
#[allow(dead_code)] // This function is called dynamically via the function registry
pub fn handle_summary_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> anyhow::Result<()> {
    fixed_fm.set_summary(captures[1].trim().to_string());
    Ok(())
}
