use poem_macros::poem_function;
use std::collections::HashMap;
use anyhow::Result;
//use regex::Regex; // Added for handle_kantspel_check
use gemini_utils::gemini_eprintln; // Use the new macro

// // #[poem_function(
// //     name = "kantspel_meme",
// //     pattern = "I will remember to use kantspel",
// //     title = "The Kantspel Meme",
// //     summary = "A meme reminding to use kantspel, inspired by Bart Simpson.",
// //     keywords = "kantspel, meme, Bart Simpson, remember, lesson",
// //     emojis = "📝🧠💡",
// //     art_generator_instructions = "Generate an image of Bart Simpson writing 'I will remember to use kantspel' repeatedly on a blackboard. The style should be classic Simpsons animation.",
// //     pending_meme_description = "This meme serves as a self-referential reminder within the system to adhere to the 'kantspel' principle for named characters and formatting."
// // )]
// // pub fn kantspel_meme_function(line: &str, captures: &regex::Captures, fixed_fm: &mut HashMap<String, String>) -> Result<()> {
// //     // This function will be called when the pattern "I will remember to use kantspel" is matched.
// //     // You can add specific logic here if needed, e.g., logging, updating internal state.
// //     gemini_eprintln!("Kantspel meme matched: {}", line);
// //     Ok(())
// // }

// #[poem_function(
//     name = "handle_kantspel_check",
//     pattern = "\\{|\\\\", // Matches literal { or \
//     title = "Kantspel Check",
//     summary = "Checks for occurrences of literal curly braces or backslashes, indicating kantspel.",
//     keywords = "kantspel, check, formatting, backslash, curly brace",
//     emojis = "👁️‍🗨️⚠️",
//     art_generator_instructions = "Generate an image of a watchful eye overseeing code, with subtle warnings around curly braces and backslashes.",
//     pending_meme_description = "This function identifies instances of kantspel (literal curly braces or backslashes) in text, serving as a reminder for proper formatting."
// )]
// pub fn handle_kantspel_check(line: &str, captures: &regex::Captures, fixed_fm: &mut HashMap<String, String>) -> Result<()>
// {
//     // This function is called when the kantspel pattern is matched.
//     // For now, we'll just print a warning.
//     gemini_eprintln!("⚠️ Kantspel detected: '{}' contains '{}'", line, captures.get(0).map_or("", |m| m.as_str()));
//     Ok(())
// }
