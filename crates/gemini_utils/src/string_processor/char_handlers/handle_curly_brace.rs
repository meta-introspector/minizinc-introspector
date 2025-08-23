use proc_macro2::TokenStream as ProcMacro2TokenStream;
use quote::quote;
use std::iter::Peekable;
use std::str::Chars;
use lazy_static::lazy_static;
use std::collections::HashMap;

use kantspel_lib::{OPEN_CURLY_BRACE, CLOSE_CURLY_BRACE};

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

// Handles {{}} and {} sequences (copied from string_processor/mod.rs)
pub fn handle_curly_braces(
    chars: &mut Peekable<Chars>,
    current_segment: &mut String,
    result_tokens: &mut ProcMacro2TokenStream,
) {
    if let Some(&OPEN_CURLY_BRACE) = chars.peek() {
        chars.next(); // consume '{'
        if let Some(&CLOSE_CURLY_BRACE) = chars.peek() {
            chars.next(); // consume '}'
            if let Some(&CLOSE_CURLY_BRACE) = chars.peek() {
                chars.next(); // consume '}'
                append_segment_and_clear(current_segment, result_tokens);
                result_tokens.extend(quote! { *EMOJIS.get("brick").unwrap_or(&"🧱") });
            } else {
                current_segment.push(OPEN_CURLY_BRACE);
                current_segment.push(OPEN_CURLY_BRACE);
                current_segment.push(CLOSE_CURLY_BRACE);
            }
        } else {
            current_segment.push(OPEN_CURLY_BRACE);
            current_segment.push(OPEN_CURLY_BRACE);
        }
    } else if let Some(&CLOSE_CURLY_BRACE) = chars.peek() {
        chars.next(); // consume '}'
        append_segment_and_clear(current_segment, result_tokens);
        result_tokens.extend(quote! { *EMOJIS.get("sparkles").unwrap_or(&"✨") });
    } else {
        current_segment.push(OPEN_CURLY_BRACE);
    }
}

pub fn handle_curly_brace_char(
    chars: &mut Peekable<Chars>,
    current_segment: &mut String,
    result_tokens: &mut ProcMacro2TokenStream,
) {
    handle_curly_braces(chars, current_segment, result_tokens);
}
