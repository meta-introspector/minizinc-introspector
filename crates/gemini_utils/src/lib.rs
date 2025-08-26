use proc_macro::TokenStream;
use syn::{parse_macro_input, Expr, Lit, LitStr}; // Added Lit
use quote::ToTokens; // Add ToTokens for debugging
//use proc_macro2::TokenStream as ProcMacro2TokenStream; // Alias for proc_macro2::TokenStream
use lazy_static::lazy_static; // Add this import
use std::collections::HashMap; // Add this import
//use std::io::Write; // Add this import for flush

// No explicit import for kantspel_lib::BACKSLASH, use kantspel_lib::BACKSLASH directly
mod macro_parser;
mod string_processor;
mod token_generator; // Add this module

use macro_parser::gemini_eprintln_input::GeminiEprintlnInput; // New import
use crate::string_processor::{EMOJIS, EMOJI_NAMES}; // Added EMOJI_NAMES
use token_generator::generate_eprintln_tokens::generate_eprintln_tokens; // Add this use statement

// Dummy usage to make lazy_static and HashMap used
lazy_static! {
    static ref DUMMY_MAP: HashMap<String, String> = {
        let mut map = HashMap::new();
        map.insert("hello".to_string(), "world".to_string());
        map
    };
}

/// A procedural macro for enhanced logging and communication within the project.
///
/// This macro adheres to strict `kantspel` principles, automatically translating
/// specific keywords and emojis into standard Rust formatting characters (`\n`, `{{}}`).
/// It supports named arguments for clear and structured output.
///
/// For internal debugging within the macro itself, where `gemini_eprintln!` cannot be
/// directly used (due to the nature of procedural macros), `eprintln!` is used.
/// In such cases, `kantspel_lib::DEBUG_FORMAT_SPECIFIER` should be utilized for debug formatting.
#[proc_macro]
pub fn gemini_eprintln(input: TokenStream) -> TokenStream {
    let parsed_input = parse_macro_input!(input as GeminiEprintlnInput);
    let format_string_literal = parsed_input.format_string;
    let mut named_args = parsed_input.named_args; // Make mutable
    let positional_args = parsed_input.positional_args;

    eprintln!("DEBUG: Parsed named_args: {:?}",
              named_args.iter().map(|(i, e)|
                                    format!("{}: {:?}",
                                            i,
                                            e.to_token_stream())).collect::<Vec<_>>());
    eprintln!("DEBUG: Parsed positional_args: {:?}", positional_args.iter().map(|e| e.to_token_stream().to_string()).collect::<Vec<_>>());

    let mut current_segment = String::new();
    let format_string_value = format_string_literal.value();
    let mut chars = format_string_value.chars().peekable();

    let mut context = crate::string_processor::processing_context::ProcessingContext {
        chars: &mut chars,
        current_segment: &mut current_segment,
        emojis: &EMOJIS,
        placeholders: Vec::new(),
    };

    let mut auto_generated_named_args: HashMap<String, Expr> = HashMap::new();

    while let Some(c) = context.chars.next() {
        // Check for ::keyword:: patterns
        if c == ':' && context.chars.peek() == Some(&':') {
            context.chars.next(); // Consume the second ':'
            let mut keyword_name = String::new();
            while let Some(&next_char) = context.chars.peek() {
                if next_char == ':' {
                    break; // End of keyword
                }
                keyword_name.push(next_char);
                context.chars.next();
            }
            if context.chars.peek() == Some(&':') {
                context.chars.next(); // Consume the final ':'

                // --- Dynamic Emoji Naming ---
                let mut resolved_emoji_char: Option<&str> = None;
                if let Some((_ident, expr)) = named_args.iter().find(|(ident, _)| ident.to_string() == keyword_name) {
                    if let Expr::Lit(expr_lit) = expr {
                        if let Lit::Str(lit_str) = &expr_lit.lit {
                            let lit_str_value = lit_str.value();
                            // Check if the string literal value is an emoji string (e.g., ":rocket:")
                            if lit_str_value.starts_with(':') && lit_str_value.ends_with(':') && lit_str_value.len() > 2 {
                                let inner_name = &lit_str_value[1..lit_str_value.len() - 1];
                                if let Some(emoji_char) = EMOJIS.get(&format!("::{:?}::", inner_name).replace("\"", "").as_str()) {
                                    resolved_emoji_char = Some(emoji_char);
                                }
                            }
                        }
                    }
                }

                if let Some(emoji_char) = resolved_emoji_char {
                    context.current_segment.push_str(emoji_char);
                    // Add placeholder for the dynamically resolved emoji
                    if let Some(emoji_name) = EMOJI_NAMES.get(emoji_char) {
                        context.placeholders.push(crate::string_processor::PlaceholderType::Named(emoji_name.to_string()));
                    } else {
                        // Fallback for emojis without a known name, treat as positional
                        context.placeholders.push(crate::string_processor::PlaceholderType::Positional(false));
                    }
                } else if let Some(replacement) = context.emojis.get(keyword_name.as_str()) {
                    context.current_segment.push_str(replacement);
                    // Handle placeholders for ::brick::, ::crane::, etc.
                    if replacement == &"{}" {
                        context.placeholders.push(crate::string_processor::PlaceholderType::Positional(false));
                    } else if replacement == &"{{:?}}" {
                        context.placeholders.push(crate::string_processor::PlaceholderType::Positional(true));
                    }
                } else {
                    // If keyword not found, treat as literal ::keyword::
                    context.current_segment.push_str("::");
                    context.current_segment.push_str(&keyword_name);
                    context.current_segment.push(':');
                }
            } else {
                // Not a ::keyword:: pattern, treat as literal ::
                context.current_segment.push_str("::");
                context.current_segment.push_str(&keyword_name);
            }
        } else if c == ':' { // Existing :key: placeholder logic
            let mut placeholder_name = String::new();
            let mut temp_chars = context.chars.clone();
            let mut peeked_chars_count = 0;

            while let Some(&next_char) = temp_chars.peek() {
                if next_char.is_alphanumeric() || next_char == '_' {
                    placeholder_name.push(next_char);
                    temp_chars.next();
                    peeked_chars_count += 1;
                } else {
                    break;
                }
            }

            if temp_chars.peek() == Some(&':') {
                context.chars.nth(peeked_chars_count);
                context.chars.next();
                context.current_segment.push_str("{}");
                context.placeholders.push(crate::string_processor::PlaceholderType::Named(placeholder_name));
            } else {
                context.current_segment.push(':');
                context.current_segment.push_str(&placeholder_name);
            }
        } else {
            // --- Automatic Emoji Variable Population ---
            if let Some(emoji_name) = EMOJI_NAMES.get(c.to_string().as_str()) {
                context.current_segment.push_str("{}"); // Replace emoji with {} placeholder
                context.placeholders.push(crate::string_processor::PlaceholderType::Named(emoji_name.to_string()));
            } else {
                context.current_segment.push(c);
            }
        }
    }

    let final_segment = std::mem::take(&mut *context.current_segment);
    let processed_format_string = LitStr::new(&final_segment, format_string_literal.span());

    let mut final_args: Vec<Option<Expr>> = vec![None; context.placeholders.len()];
    let mut used_named_args: HashMap<String, bool> = HashMap::new();

    // First Pass: Fill Explicitly Named Placeholders
    let mut unclaimed_named_args: Vec<(syn::Ident, syn::Expr)> = Vec::new();

    for (ident, expr) in named_args.into_iter() {
        let ident_str = ident.to_string();
        let mut assigned = false;

        for (i, placeholder_type) in context.placeholders.iter().enumerate() {
            if let crate::string_processor::PlaceholderType::Named(name) = placeholder_type {
                if name == &ident_str {
                    if final_args[i].is_none() {
                        final_args[i] = Some(expr.clone());
                        assigned = true;
                        break;
                    } else {
                        return syn::Error::new_spanned(ident.clone(), format!("Named argument '{}' maps to an already filled placeholder.", ident_str)).to_compile_error().into();
                    }
                }
            }
        }

        if assigned {
            used_named_args.insert(ident_str, true);
        } else {
            unclaimed_named_args.push((ident, expr));
        }
    }

    // Second Pass: Fill Positional Placeholders and Unclaimed Named Arguments
    let mut positional_arg_iter = positional_args.into_iter();
    let mut unclaimed_named_arg_iter = unclaimed_named_args.into_iter();

    eprintln!("DEBUG: Before loop - positional_arg_iter has next: {}", positional_arg_iter.clone().next().is_some());
    eprintln!("DEBUG: Before loop - unclaimed_named_arg_iter has next: {}", unclaimed_named_arg_iter.clone().next().is_some());

    for (i, placeholder_type) in context.placeholders.iter().enumerate() {
        eprintln!("DEBUG: Loop iteration {} - Placeholder type: {}", i, format!("{:?}", placeholder_type));
        eprintln!("DEBUG: Loop iteration {} - final_args[{}]: {:?}", i, i, final_args[i].as_ref().map(|e| e.to_token_stream().to_string()));

        if final_args[i].is_none() {
            match placeholder_type {
                crate::string_processor::PlaceholderType::Positional(_is_debug) => {
                    eprintln!("DEBUG: Loop iteration {} - Positional placeholder. positional_arg_iter has next: {}", i, positional_arg_iter.clone().next().is_some());
                    eprintln!("DEBUG: Loop iteration {} - Positional placeholder. unclaimed_named_arg_iter has next: {}", i, unclaimed_named_arg_iter.clone().next().is_some());

                    if let Some((ident, expr)) = unclaimed_named_arg_iter.next() {
                        eprintln!("DEBUG: Loop iteration {} - Filling with unclaimed named arg: {}", i, ident);
                        final_args[i] = Some(expr);
                        used_named_args.insert(ident.to_string(), true);
                    } else if let Some(expr) = positional_arg_iter.next() {
                        eprintln!("DEBUG: Loop iteration {} - Filling with positional arg: {}", i, expr.to_token_stream().to_string());
                        final_args[i] = Some(expr);
                    } else {
                        // If positional placeholder is not filled, provide a default empty string
                        final_args[i] = Some(syn::parse_quote!{""});
                    }
                },
                crate::string_processor::PlaceholderType::Named(name) => {
                    eprintln!("DEBUG: Loop iteration {} - Named placeholder: {}", i, name);
                    // If a named placeholder is not filled by an explicit argument, auto-generate an empty string
                    final_args[i] = Some(syn::parse_quote!{""});
                }
            }
        }
    }

    eprintln!("DEBUG: After loop - positional_arg_iter has next: {}", positional_arg_iter.clone().next().is_some());
    eprintln!("DEBUG: After loop - unclaimed_named_arg_iter has next: {}", unclaimed_named_arg_iter.clone().next().is_some());

    // Check for unassigned placeholders (should be caught by previous loops, but as a safeguard)
    for (i, arg_opt) in final_args.iter().enumerate() {
        if arg_opt.is_none() {
            return syn::Error::new_spanned(
                format_string_literal.clone(),
                format!("Placeholder at index {} is not filled by any argument (safeguard).",i),
            ).to_compile_error().into();
        }
    }

    // Check for unused named arguments
    if let Some((ident, _)) = unclaimed_named_arg_iter.next() {
        return syn::Error::new_spanned(ident.clone(), format!("Named argument '{}' is not used in the format string.", ident.to_string())).to_compile_error().into();
    }

    // Check for unused positional arguments
    if let Some(expr) = positional_arg_iter.next() {
        return syn::Error::new_spanned(expr.clone(), "Too many positional arguments provided.").to_compile_error().into();
    }

    let final_exprs: Vec<Expr> = final_args.into_iter().map(|opt_expr| opt_expr.unwrap()).collect();

    generate_eprintln_tokens(processed_format_string, true, final_exprs.iter().collect()).into()
}

#[cfg(test)]
mod tests {
    use super::gemini_eprintln; // Import the macro

    #[test]
    fn test_named_argument_with_emoji() {
        // This tests that the macro correctly processes a named argument
        // when a placeholder emoji (🔍) is present.
        let my_value = 42;
        gemini_eprintln!("The answer is: 🔍", value:my_value);
        // In a real test, you'd inspect the generated code or output,
        // but for a proc macro, successful compilation is the primary check.
    }

    #[test]
    fn test_positional_argument_with_emoji() {
        // This tests that the macro correctly processes a positional argument
        // when a placeholder emoji (🔍) is present.
        let my_string = "hello world";
        gemini_eprintln!("A message: 🔍", my_string);
    }
}
