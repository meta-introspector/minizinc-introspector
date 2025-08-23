use proc_macro::TokenStream;
use syn::{parse_macro_input, Expr, ExprLit, LitStr};
use quote::{quote, ToTokens};
use proc_macro2::TokenStream as ProcMacro2TokenStream; // Alias for proc_macro2::TokenStream
use lazy_static::lazy_static; // Add this import
use std::collections::HashMap; // Add this import
use proc_macro2::Span; // Add this import

// No explicit import for kantspel_lib::BACKSLASH, use kantspel_lib::BACKSLASH directly
mod macro_parser;
mod string_processor;
mod token_generator; // Add this module

use macro_parser::comma_separated_exprs::CommaSeparatedExprs;
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

    let input_args = parse_macro_input!(input as CommaSeparatedExprs).exprs;

    if input_args.is_empty() {
        return quote! { eprintln!(); }.into();
    }

    let format_expr = &input_args[0];
    let other_args = input_args.iter().skip(1).collect::<Vec<_>>();

    let (processed_format_string_literal, is_literal_string) = if let Expr::Lit(ExprLit { lit: syn::Lit::Str(s), .. }) = format_expr {
        let original_string = s.value();

        // --- KANTSPEL TRANSLATION GOES HERE ---
        // This is where keywords/emojis like "sparkles" or "brickwall" will be translated
        // into \n, {}, or {{}} for the underlying eprintln! macro.
        // The process_char_for_emojis function will be responsible for this.
        // The current_segment will hold the translated string.

        let mut current_segment = String::new();

        let mut chars = original_string.chars().peekable();
        let mut context = string_processor::processing_context::ProcessingContext {
            chars: &mut chars,
            current_segment: &mut current_segment,
            emojis: &EMOJIS,
        };
        while let Some(c) = context.chars.next() {
            process_char_for_emojis(c, &mut context);
        }

        (LitStr::new(&current_segment, s.span()), true) // Create LitStr from current_segment
    } else {
        // Debug print for non-literal format string, using Span
        eprintln!("DEBUG: Non-literal format string at span: {:?}", Span::call_site());
        (LitStr::new(&format_expr.to_token_stream().to_string(), proc_macro2::Span::call_site()), false) // Convert Expr to LitStr
    };

    generate_eprintln_tokens(processed_format_string_literal, is_literal_string, other_args).into()
}
