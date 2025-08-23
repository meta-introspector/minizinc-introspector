use proc_macro2::TokenStream as ProcMacro2TokenStream;
use quote::quote;
use std::iter::Peekable;
use std::str::Chars;
use lazy_static::lazy_static;
use std::collections::HashMap;

// Re-import EMOJIS and append_segment_and_clear from string_processor/mod.rs
// This is a temporary measure. Ideally, these would be passed as arguments or
// accessed through a shared context if they are truly atomic and independent.
// For now, to make the extracted function compile, I'll re-declare them.
// The overall refactoring will address proper passing of these.

// Placeholder for EMOJIS. This should ideally come from a generated file.
lazy_static! {
    pub static ref EMOJIS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("return", "⏎");
        map.insert("brick", "🧱");
        map.insert("sparkles", "✨");
        map
    };
}

// Helper to append segment and clear (copied from string_processor/mod.rs)
pub fn append_segment_and_clear(
    current_segment: &mut String,
    result_tokens: &mut ProcMacro2TokenStream,
) {
    if !current_segment.is_empty() {
        result_tokens.extend(quote! { #current_segment });
        current_segment.clear();
    }
}

// Handles \n sequence (copied from string_processor/mod.rs)
pub fn handle_backslash_n(
    chars: &mut Peekable<Chars>,
    current_segment: &mut String,
    result_tokens: &mut ProcMacro2TokenStream,
) {
    chars.next(); // consume 'n'
    append_segment_and_clear(current_segment, result_tokens);
    result_tokens.extend(quote! { *EMOJIS.get("return").unwrap_or(&"⏎") });
}

pub fn handle_backslash_char(
    c: char,
    chars: &mut Peekable<Chars>,
    current_segment: &mut String,
    result_tokens: &mut ProcMacro2TokenStream,
) {
    if let Some('n') = chars.peek() {
        handle_backslash_n(chars, current_segment, result_tokens);
    } else {
        current_segment.push(c);
    }
}
