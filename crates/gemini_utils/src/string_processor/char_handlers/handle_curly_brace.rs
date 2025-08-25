//use std::iter::Peekable;
//use std::str::Chars;
///use std::collections::HashMap;

use kantspel_lib::{OPEN_CURLY_BRACE, CLOSE_CURLY_BRACE};
use super::super::processing_context::ProcessingContext;

#[allow(dead_code)]
pub fn handle_curly_braces(
    context: &mut ProcessingContext,
) {
    // If it's an opening curly brace
    if context.chars.peek() == Some(&OPEN_CURLY_BRACE) {
        context.chars.next(); // Consume the '{'
        if context.chars.peek() == Some(&OPEN_CURLY_BRACE) {
            context.chars.next(); // Consume the second '{'
            context.current_segment.push_str("{{"); // Push literal {{
        } else {
            context.current_segment.push(OPEN_CURLY_BRACE); // Push literal {
        }
    } else if context.chars.peek() == Some(&CLOSE_CURLY_BRACE) {
        context.chars.next(); // Consume the '}'
        context.current_segment.push(CLOSE_CURLY_BRACE); // Push literal }
    }
}

#[allow(dead_code)]
pub fn handle_curly_brace_char(
    context: &mut ProcessingContext,
) {
    handle_curly_braces(context);
}
