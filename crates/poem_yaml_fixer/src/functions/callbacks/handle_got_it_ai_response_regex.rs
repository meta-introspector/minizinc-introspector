use anyhow::Result;
use std::collections::HashMap;
use poem_traits::PoemFrontMatterTrait;
use poem_macros::poem_function;
use crate::functions::generate_regex::generate_generalized_regex;

#[poem_function(
    name = "ai_response",
    pattern = r#"^"Got it"\s*🎼\s*—\s*(.*)"#,
    title = "AI Response",
    summary = "Extracts AI response content.",
    keywords = "AI, response, chat",
    emojis = "🤖",
    art_generator_instructions = "Generate an image of an AI chatbot responding.",
    pending_meme_description = "This callback extracts AI response content."
)]
pub fn handle_got_it_ai_response_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut dyn PoemFrontMatterTrait) -> Result<()> {
    fixed_fm.raw_meme_lines.get_or_insert_with(Vec::new).push(format!("ai_response: {}", captures[1].trim()));
    Ok(())
}