use proc_macro::TokenStream;
use syn::{parse_macro_input, Expr,
	  //ExprLit,
	  LitStr};
//use quote::{quote, ToTokens};
use proc_macro2::TokenStream as ProcMacro2TokenStream; // Alias for proc_macro2::TokenStream
use lazy_static::lazy_static; // Add this import
use std::collections::HashMap; // Add this import
//use proc_macro2::Span; // Add this import

// No explicit import for kantspel_lib::BACKSLASH, use kantspel_lib::BACKSLASH directly
mod macro_parser;
mod string_processor;
mod token_generator; // Add this module

use macro_parser::gemini_eprintln_input::GeminiEprintlnInput; // New import
use string_processor::{process_char_for_emojis, EMOJIS};
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
/// specific keywords and emojis into standard Rust formatting characters (`\n`, `{}`).
/// It supports named arguments for clear and structured output.
///
/// For internal debugging within the macro itself, where `gemini_eprintln!` cannot be
/// directly used (due to the nature of procedural macros), `eprintln!` is used.
/// In such cases, `kantspel_lib::DEBUG_FORMAT_SPECIFIER` should be utilized for debug formatting.
#[proc_macro]
pub fn gemini_eprintln(input: TokenStream) -> TokenStream {
    // IMPORTANT: Using `eprintln!` directly here because `gemini_eprintln!` cannot be used
    // within its own definition (it's a procedural macro). These prints will appear
    // during the compilation of crates that use `gemini_utils`.

    // Debug print for the input TokenStream using proc_macro2::TokenStream
    eprintln!("DEBUG: Input TokenStream (proc_macro2): {:?}", ProcMacro2TokenStream::from(input.clone()));

    // Debug print for kantspel_lib usage
    eprintln!("DEBUG: Kantspel backslash constant: {:?}", kantspel_lib::BACKSLASH);

    // Old parsing logic (commented out for refactoring)
    // let input_args = parse_macro_input!(input as CommaSeparatedExprs).exprs;
    // if input_args.is_empty() {
    //     return quote! { eprintln!(); }.into();
    // }
    // let format_expr = &input_args[0];
    // let other_args = input_args.iter().skip(1).collect::<Vec<_>>();

    // New parsing logic for GeminiEprintlnInput
    let parsed_input = parse_macro_input!(input as GeminiEprintlnInput);
    let format_string_literal = parsed_input.format_string;
    let named_args = parsed_input.named_args; // No longer mutable here
    let positional_args = parsed_input.positional_args; // No longer mutable here

    let mut current_segment = String::new();
    let format_string_value = format_string_literal.value();
    let mut chars = format_string_value.chars().peekable();

    let mut context = string_processor::processing_context::ProcessingContext {
        chars: &mut chars,
        current_segment: &mut current_segment,
        emojis: &EMOJIS,
        placeholders: Vec::new(), // Initialize with new PlaceholderType
    };

    while let Some(c) = context.chars.next() {
        // Check for named argument placeholder like :key:
        if c == ':' {
            let mut placeholder_name = String::new();
            let mut temp_chars = context.chars.clone(); // Clone to peek ahead without consuming
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
                // It's a valid :key: placeholder
                context.chars.nth(peeked_chars_count); // Consume the chars for the placeholder name
                context.chars.next(); // Consume the closing ':'
                context.current_segment.push_str("{}"); // Replace :key: with {}
                context.placeholders.push(string_processor::PlaceholderType::Named(placeholder_name)); // Store named placeholder
            } else {
                // Not a named argument placeholder, treat as literal ':' and process normally
                context.current_segment.push(':');
                // Re-process the characters that were part of the potential placeholder name
                // This is a bit tricky, as process_char_for_emojis expects to consume from context.chars
                // For simplicity, let's just push them back as literals for now.
                context.current_segment.push_str(&placeholder_name);
            }
        } else {
            // Process other characters normally
            process_char_for_emojis(c, &mut context);
        }
    }

    let final_segment = std::mem::take(context.current_segment); // Take ownership of the string
    let processed_format_string = LitStr::new(&final_segment, format_string_literal.span());

    // --- NEW ARGUMENT MAPPING LOGIC ---
    let mut final_args: Vec<Option<Expr>> = vec![None; context.placeholders.len()];
    let mut used_named_args: HashMap<String, bool> = HashMap::new();

    // First Pass: Fill Explicitly Named Placeholders
    let mut unclaimed_named_args: Vec<(syn::Ident, syn::Expr)> = Vec::new();

    for (ident, expr) in named_args.into_iter() {
        let ident_str = ident.to_string();
        let mut assigned = false;

        for (i, placeholder_type) in context.placeholders.iter().enumerate() {
            if let string_processor::PlaceholderType::Named(name) = placeholder_type {
                if name == &ident_str {
                    if final_args[i].is_none() {
                        final_args[i] = Some(expr.clone()); // Clone expr as it might be used multiple times
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
            unclaimed_named_args.push((ident, expr)); // Push back to be processed as unclaimed
        }
    }

    // Second Pass: Fill Positional Placeholders and Unclaimed Named Arguments
    let mut positional_arg_iter = positional_args.into_iter();
    let mut unclaimed_named_arg_iter = unclaimed_named_args.into_iter();

    for (i, placeholder_type) in context.placeholders.iter().enumerate() {
        if final_args[i].is_none() {
            match placeholder_type {
                string_processor::PlaceholderType::Positional(_is_debug) => {
                    if let Some((ident, expr)) = unclaimed_named_arg_iter.next() {
                        final_args[i] = Some(expr);
                        used_named_args.insert(ident.to_string(), true);
                    } else if let Some(expr) = positional_arg_iter.next() {
                        final_args[i] = Some(expr);
                    } else {
                        return syn::Error::new_spanned(format_string_literal.clone(), format!("Positional placeholder at index {} is not filled by any argument.", i)).to_compile_error().into();
                    }
                },
                string_processor::PlaceholderType::Named(name) => {
                    // This case means an explicit named placeholder was not filled in the first pass.
                    // This should be an error.
                    return syn::Error::new_spanned(format_string_literal.clone(), format!("Named placeholder '{}' at index {} is not filled by any argument.", name, i)).to_compile_error().into();
                }
            }
        }
    }

    // Check for unassigned placeholders (should be caught by previous loops, but as a safeguard)
    for (i, arg_opt) in final_args.iter().enumerate() {
        if arg_opt.is_none() {
            return syn::Error::new_spanned(format_string_literal.clone(), format!("Placeholder at index {} is not filled by any argument (safeguard).", i)).to_compile_error().into();
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
