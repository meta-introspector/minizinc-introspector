use proc_macro2::TokenStream as ProcMacro2TokenStream;
use quote::quote;
use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;
use lazy_static::lazy_static; // Import lazy_static

use crate::kantspel::{BACKSLASH, OPEN_CURLY_BRACE, CLOSE_CURLY_BRACE}; // Import kantspel constants

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

// Helper to append segment and clear
pub fn append_segment_and_clear(
    current_segment: &mut String,
    result_tokens: &mut ProcMacro2TokenStream,
) {
    if !current_segment.is_empty() {
        result_tokens.extend(quote! { #current_segment });
        current_segment.clear();
    }
}

// Handles \n sequence
pub fn handle_backslash_n(
    chars: &mut Peekable<Chars>,
    current_segment: &mut String,
    result_tokens: &mut ProcMacro2TokenStream,
) {
    chars.next(); // consume 'n'
    append_segment_and_clear(current_segment, result_tokens);
    result_tokens.extend(quote! { *EMOJIS.get("return").unwrap_or(&"⏎") });
}

// Handles {{}} and {} sequences
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

// Main character processing logic
pub fn process_char_for_emojis(
    c: char,
    chars: &mut Peekable<Chars>,
    current_segment: &mut String,
    result_tokens: &mut ProcMacro2TokenStream,
) {
    match c {
        BACKSLASH => {
            if let Some('n') = chars.peek() { // 'n' is not in kantspel, so keep as is
                handle_backslash_n(chars, current_segment, result_tokens);
            } else {
                current_segment.push(c);
            }
        },
        OPEN_CURLY_BRACE => {
            handle_curly_braces(chars, current_segment, result_tokens);
        },
        _ => current_segment.push(c),
    }
}