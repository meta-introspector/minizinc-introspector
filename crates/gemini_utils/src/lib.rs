use proc_macro::TokenStream;
use syn::{parse_macro_input, Expr, ExprLit, LitStr};
use quote::{quote, ToTokens};
use proc_macro2::TokenStream as ProcMacro2TokenStream;
use lazy_static::lazy_static; // Add this import
use std::collections::HashMap; // Add this import
use proc_macro2::Span; // Add this import

use kantspel_lib::*; // Corrected import for kantspel_lib
mod macro_parser;
mod string_processor;
mod token_generator; // Add this module

use macro_parser::comma_separated_exprs::CommaSeparatedExprs;
use string_processor::{process_char_for_emojis, EMOJIS};
use token_generator::generate_eprintln_tokens::generate_eprintln_tokens; // Add this use statement

// Define EMOJIS here


#[proc_macro]
pub fn gemini_eprintln(input: TokenStream) -> TokenStream {
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
        (LitStr::new(&format_expr.to_token_stream().to_string(), proc_macro2::Span::call_site()), false) // Convert Expr to LitStr
    };

    generate_eprintln_tokens(processed_format_string_literal, is_literal_string, other_args).into()
}
