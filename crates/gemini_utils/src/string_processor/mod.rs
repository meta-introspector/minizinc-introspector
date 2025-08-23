use proc_macro2::TokenStream as ProcMacro2TokenStream;
use std::iter::Peekable;
use std::str::Chars;
use lazy_static::lazy_static; // Add this import
use std::collections::HashMap; // Add this import

use kantspel_lib::{BACKSLASH, OPEN_CURLY_BRACE};

pub mod char_handlers;
pub mod segment_appender;
pub mod processing_context;



// Define EMOJIS here
lazy_static! {
    pub(crate) static ref EMOJIS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        // Map emojis and keywords to their corresponding characters for LLM readability
        map.insert("✨", "\\n"); // sparkles emoji to newline
        map.insert("sparkles", "\\n"); // sparkles keyword to newline
        map.insert("🧱", "{}"); // brick emoji to curly braces
        map.insert("brick", "{}"); // brick keyword to curly braces
        map.insert("🏗️", "{{}}"); // brick emoji to curly braces
        map.insert("building-construction", "{{}}"); // brick keyword to curly braces
        map.insert("⏎", "return"); // return emoji to return keyword
        map
    };
}

use crate::string_processor::processing_context::ProcessingContext; // Add this import

// Main character processing logic
pub fn process_char_for_emojis(
    c: char,
    context: &mut ProcessingContext,
) {
    // Try to match multi-character emojis/keywords first
    for (keyword, replacement) in context.emojis.iter() {
        if keyword.len() > 1 && context.chars.as_str().starts_with(keyword) {
            // Consume the characters for the keyword
            for _ in 0..keyword.len() {
                context.chars.next();
            }
            context.current_segment.push_str(replacement);
            return; // Keyword matched and processed, return
        }
    }

    // If no multi-character keyword matched, process single characters
    match c {
        BACKSLASH => {
            char_handlers::handle_backslash::handle_backslash_char(c, context);
        },
        OPEN_CURLY_BRACE => {
            char_handlers::handle_curly_brace::handle_curly_brace_char(context);
        },
        _ => char_handlers::handle_other_char::handle_other_char(c, context),
    }
}
