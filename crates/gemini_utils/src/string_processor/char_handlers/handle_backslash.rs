use proc_macro2::TokenStream as ProcMacro2TokenStream;
use quote::quote;
use std::iter::Peekable;
use std::str::Chars;
use std::collections::HashMap; // Add this import

use super::super::segment_appender::append_segment_and_clear;

pub fn handle_backslash_n(
    chars: &mut Peekable<Chars>,
    current_segment: &mut String,
    result_tokens: &mut ProcMacro2TokenStream,
    emojis: &HashMap<&'static str, &'static str>, // Add emojis parameter
) {
    chars.next(); // consume 'n'
    append_segment_and_clear(current_segment, result_tokens);
    result_tokens.extend(quote! { *emojis.get("return").unwrap_or(&"⏎") }); // Use emojis parameter
}

pub fn handle_backslash_char(
    c: char,
    chars: &mut Peekable<Chars>,
    current_segment: &mut String,
    result_tokens: &mut ProcMacro2TokenStream,
    emojis: &HashMap<&'static str, &'static str>, // Add emojis parameter
) {
    if let Some('n') = chars.peek() {
        handle_backslash_n(chars, current_segment, result_tokens, emojis); // Pass emojis
    } else {
        current_segment.push(c);
    }
}