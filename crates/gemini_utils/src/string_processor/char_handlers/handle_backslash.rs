//use std::iter::Peekable;
//use std::str::Chars;
//use std::collections::HashMap;

use super::super::processing_context::ProcessingContext;

pub fn handle_backslash_n(
    context: &mut ProcessingContext,
) {
    context.chars.next(); // consume 'n'
    context.current_segment.push_str(context.emojis.get("return").unwrap_or(&"⏎"));
}

pub fn handle_backslash_char(
    c: char,
    context: &mut ProcessingContext,
) {
    if let Some('n') = context.chars.peek() {
        handle_backslash_n(context);
    } else {
        context.current_segment.push(c);
    }
}
