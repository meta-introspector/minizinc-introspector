use proc_macro2::TokenStream as ProcMacro2TokenStream;
use quote::quote;
use std::iter::Peekable;
use std::str::Chars;
use std::collections::HashMap; // Add this import

use kantspel_lib::{OPEN_CURLY_BRACE, CLOSE_CURLY_BRACE};
use super::super::segment_appender::append_segment_and_clear;

pub fn handle_curly_braces(
    chars: &mut Peekable<Chars>,
    current_segment: &mut String,
    result_tokens: &mut ProcMacro2TokenStream,
    emojis: &HashMap<&'static str, &'static str>, // Add emojis parameter
) {
    if let Some(&OPEN_CURLY_BRACE) = chars.peek() {
        chars.next(); // consume '{'
        if let Some(&CLOSE_CURLY_BRACE) = chars.peek() {
            chars.next(); // consume '}'
            if let Some(&CLOSE_CURLY_BRACE) = chars.peek() {
                chars.next(); // consume '}'
                append_segment_and_clear(current_segment, result_tokens);
                result_tokens.extend(quote! { *emojis.get("brick").unwrap_or(&"🧱") }); // Use emojis parameter
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
        result_tokens.extend(quote! { *emojis.get("sparkles").unwrap_or(&"✨") }); // Use emojis parameter
    } else {
        current_segment.push(OPEN_CURLY_BRACE);
    }
}

pub fn handle_curly_brace_char(
    chars: &mut Peekable<Chars>,
    current_segment: &mut String,
    result_tokens: &mut ProcMacro2TokenStream,
    emojis: &HashMap<&'static str, &'static str>, // Add emojis parameter
) {
    handle_curly_braces(chars, current_segment, result_tokens, emojis); // Pass emojis
}