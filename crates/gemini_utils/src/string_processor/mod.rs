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
        map.insert("return", "⏎");
        map.insert("brick", "🧱");
        map.insert("sparkles", "✨");
        map
    };
}

use crate::string_processor::processing_context::ProcessingContext; // Add this import

// Main character processing logic
pub fn process_char_for_emojis(
    c: char,
    context: &mut ProcessingContext,
) {
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
