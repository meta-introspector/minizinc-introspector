//use std::iter::Peekable;
//use std::str::Chars;
///use std::collections::HashMap;

use kantspel_lib::{OPEN_CURLY_BRACE, CLOSE_CURLY_BRACE};
use super::super::processing_context::ProcessingContext;

pub fn handle_curly_braces(
    context: &mut ProcessingContext,
) {
    // Check for {{}} (brick emoji)
    if context.chars.peek() == Some(&OPEN_CURLY_BRACE) {
        let _ = context.chars.next(); // consume first '{'
        if context.chars.peek() == Some(&CLOSE_CURLY_BRACE) {
            let _ = context.chars.next(); // consume first '}'
            if context.chars.peek() == Some(&CLOSE_CURLY_BRACE) {
                let _ = context.chars.next(); // consume second '}'
                context.current_segment.push_str(context.emojis.get("brick").unwrap_or(&"🧱"));
                return;
            }
        }
        // If not {{}}, push the initial '{' back as a literal for eprintln!
        context.current_segment.push(OPEN_CURLY_BRACE);
        context.current_segment.push(OPEN_CURLY_BRACE); // Push another '{' to make it '{{' for eprintln!
        return;
    }

    // Check for {} (sparkles emoji)
    if context.chars.peek() == Some(&CLOSE_CURLY_BRACE) {
        let _ = context.chars.next(); // consume '}'
        context.current_segment.push_str(context.emojis.get("sparkles").unwrap_or(&"✨"));
        return;
    }

    // If it's just a single '{' or '}' not part of an emoji sequence,
    // push it as a literal character.
    context.current_segment.push(OPEN_CURLY_BRACE);
}

pub fn handle_curly_brace_char(
    context: &mut ProcessingContext,
) {
    handle_curly_braces(context);
}
