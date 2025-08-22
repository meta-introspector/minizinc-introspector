use poem_traits::PoemFrontMatterTrait;

#[poem_macros::poem_function(
    name = "art_generator_instructions_field",
    pattern = r"art_generator_instructions:\s*(.*)",
    title = "Art Generator Instructions",
    summary = "Extracts and sets instructions for art generation.",
    keywords = "art, generation, instructions, metadata",
    emojis = "🎨✍️",
    art_generator_instructions = "Generate an image of a robot painting.",
    pending_meme_description = "This is a pending description for the art generator instructions field."
)]

pub fn handle_art_generator_instructions_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> anyhow::Result<()> {
    fixed_fm.set_art_generator_instructions(captures[1].trim().to_string());
    Ok(())
}
