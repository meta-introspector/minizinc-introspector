use proc_macro2::TokenStream as ProcMacro2TokenStream;
use std::iter::Peekable;
use std::str::Chars;

use kantspel_lib::{BACKSLASH, OPEN_CURLY_BRACE};

pub mod char_handlers;
pub mod segment_appender;

use char_handlers::{handle_backslash, handle_curly_brace, handle_other_char};
use segment_appender::append_segment_and_clear;

// Main character processing logic
pub fn process_char_for_emojis(
    c: char,
    chars: &mut Peekable<Chars>,
    current_segment: &mut String,
    result_tokens: &mut ProcMacro2TokenStream,
) {
    match c {
        BACKSLASH => {
            handle_backslash::handle_backslash_char(c, chars, current_segment, result_tokens);
        },
        OPEN_CURLY_BRACE => {
            handle_curly_brace::handle_curly_brace_char(chars, current_segment, result_tokens);
        },
        _ => handle_other_char::handle_other_char(c, chars, current_segment, result_tokens),
    }
}
