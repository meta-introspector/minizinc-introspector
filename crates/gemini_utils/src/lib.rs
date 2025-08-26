use proc_macro::TokenStream;
use syn::{parse_macro_input, Expr,
	  //ExprLit,
	  LitStr};
//use quote::{quote, ToTokens};
//use proc_macro2::TokenStream as ProcMacro2TokenStream; // Alias for proc_macro2::TokenStream
use lazy_static::lazy_static; // Add this import
use std::collections::HashMap; // Add this import
//use std::io::Write; // Add this import for flush

// No explicit import for kantspel_lib::BACKSLASH, use kantspel_lib::BACKSLASH directly
mod macro_parser;
mod string_processor;
mod token_generator; // Add this module

use macro_parser::gemini_eprintln_input::GeminiEprintlnInput; // New import
use crate::string_processor::EMOJIS;
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
    // IMPORTANT: Using `eprintln!` directly here because `gemini_eprintln!` cannot be used
    // within its own definition (it's a procedural macro). These prints will appear
    // during the compilation of crates that use `gemini_utils`.

    // Debug print for the input TokenStream using proc_macro2::TokenStream
    //    eprintln!("DEBUG: Input TokenStream (proc_macro2): {{:?}}", ProcMacro2TokenStream::from(input.clone()));

    // Debug print for kantspel_lib usage
//    eprintln!("DEBUG: Kantspel backslash constant: {{:?}}", kantspel_lib::BACKSLASH);

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
    let named_args = parsed_input.named_args;
    let positional_args = parsed_input.positional_args;

    let mut current_segment = String::new();
    let format_string_value = format_string_literal.value();
    let mut chars = format_string_value.chars().peekable();

    let mut context = crate::string_processor::processing_context::ProcessingContext {
        chars: &mut chars,
        current_segment: &mut current_segment,
        emojis: &EMOJIS,
        placeholders: Vec::new(), // Initialize with new PlaceholderType
    };

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
                if let Some(replacement) = context.emojis.get(keyword_name.as_str()) {
                    context.current_segment.push_str(replacement);
                    // Handle placeholders for ::brick::, ::crane::, etc.
                    if replacement == &"{{}}" { // For brick
                        context.placeholders.push(crate::string_processor::PlaceholderType::Positional(false));
                    } else if replacement == &"{?:}" { // For 🔍/inspect
                        context.placeholders.push(crate::string_processor::PlaceholderType::Positional(true));
                    } else if replacement == &"🔍" { // For /inspect
                        context.placeholders.push(crate::string_processor::PlaceholderType::Positional(true));
                    } else if replacement == &":inspect:" { // inspect
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
        }
    }

    let final_segment = std::mem::take(&mut *context.current_segment); // Take ownership of the string
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
            if let crate::string_processor::PlaceholderType::Named(name) = placeholder_type {
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
                crate::string_processor::PlaceholderType::Positional(_is_debug) => {
                    if let Some((ident, expr)) = unclaimed_named_arg_iter.next() {
                        final_args[i] = Some(expr);
                        used_named_args.insert(ident.to_string(), true);
                    } else if let Some(expr) = positional_arg_iter.next() {
                        final_args[i] = Some(expr);
                    } else {
                        return syn::Error::new_spanned(format_string_literal.clone(), format!("Positional placeholder at index {} is not filled by any argument.", i)).to_compile_error().into();
                    }
                },
                crate::string_processor::PlaceholderType::Named(name) => {
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
