#[derive(Debug)]
pub enum PlaceholderType {
    Named(String), // For :key: placeholders
    Positional(bool), // For {} or {:?} (bool indicates if it's debug format)
}

//use proc_macro2::TokenStream as ProcMacro2TokenStream;
//use std::iter::Peekable;
//use std::str::Chars;
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
        map.insert("::variable::", "{}");
	map.insert(":::brick:::", "{}"); // New: the read brick emoji maps to {}	
	map.insert("::quoted-variable::", "{{}}"); 
	map.insert(":::crane:::", "{{}}");	
        map.insert("::newline::", "\n"); // New: ::newline:: for newline
        map.insert("::sparkles::", "✨"); // New: ::sparkles:: for literal ✨ emoji
        map.insert("🔍", "{:?}"); // magnifying glass emoji to debug format
	//map.insert(":::inspect:::", "{:?}"); // the same as map.insert("🔍", "{:?}");
        map.insert("::inspect::", "{:?}"); // inspect keyword to debug format
	map.insert("quoted-inspect", "{{:?}}"); // inspect keyword to debug format

	// return
	map
    };
}

use crate::string_processor::processing_context::ProcessingContext; // Add this import

// Main character processing logic
#[allow(dead_code)]
pub fn process_char_for_emojis(
    c: char,
    context: &mut ProcessingContext,
) {
    // Try to match multi-character emojis/keywords first
    // Iterate over sorted keywords (longest first) to ensure correct matching
    let mut sorted_emojis: Vec<(&&str, &&str)> = context.emojis.iter().collect();
    sorted_emojis.sort_by(|a, b| b.0.len().cmp(&a.0.len()));

    for (keyword, replacement) in sorted_emojis {
        if keyword.len() > 1 {
            let mut temp_chars = context.chars.clone();
            let mut matches = true;
            for k_char in keyword.chars() {
                if let Some(c) = temp_chars.next() {
                    if c != k_char {
                        matches = false;
                        break;
                    }
                } else {
                    matches = false;
                    break;
                }
            }
            if matches {
                // Consume the characters for the keyword from the original iterator
                for _ in 0..keyword.len() {
                    context.chars.next();
                }
                context.current_segment.push_str(replacement);

                // ---
                // NEW LOGIC
                // ---
                if replacement == &"{}" { // For brick
                    context.placeholders.push(PlaceholderType::Positional(false));
                } else if replacement == &"{{:?}}" { // For 🔍/inspect
                    context.placeholders.push(PlaceholderType::Positional(true));
                }
                // ---
                // END NEW LOGIC
                // ---

                return; // Keyword matched and processed, return
            }
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

#[allow(dead_code)]
pub fn clean_string_for_regex(input: &str) -> String {
    let mut cleaned = input.trim().to_string();
    // Remove the black diamond character (U+25C6)
    cleaned = cleaned.replace('◆', "");
    // Add more cleaning rules as needed
    cleaned
}