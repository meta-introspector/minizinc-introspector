use proc_macro::TokenStream;
use syn::{parse_macro_input, Expr, ExprLit};
use quote::quote;
use proc_macro2::TokenStream as ProcMacro2TokenStream;
use lazy_static::lazy_static; // Add this import
use std::collections::HashMap; // Add this import

use kantspel_lib::*;
mod macro_parser;
mod string_processor;

use macro_parser::comma_separated_exprs::CommaSeparatedExprs;
use string_processor::{process_char_for_emojis, append_segment_and_clear};

// Define EMOJIS here
lazy_static! {
    pub static ref EMOJIS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("return", "⏎");
        map.insert("brick", "🧱");
        map.insert("sparkles", "✨");
        map
    };
}

#[proc_macro]
pub fn gemini_eprintln(input: TokenStream) -> TokenStream {
    let input_args = parse_macro_input!(input as CommaSeparatedExprs).exprs;

    if input_args.is_empty() {
        return quote! { eprintln!(); }.into();
    }

    let format_expr = &input_args[0];
    let other_args = input_args.iter().skip(1).collect::<Vec<_>>();

    let (processed_format_string_tokens, is_literal_string) = if let Expr::Lit(ExprLit { lit: syn::Lit::Str(s), .. }) = format_expr {
        let original_string = s.value();

        let mut result_tokens = ProcMacro2TokenStream::new();
        let mut current_segment = String::new();

        let mut chars = original_string.chars().peekable();
        while let Some(c) = chars.next() {
            process_char_for_emojis(c, &mut chars, &mut current_segment, &mut result_tokens, &EMOJIS); // Pass &EMOJIS
        }
        append_segment_and_clear(&mut current_segment, &mut result_tokens);

        (result_tokens, true)
    } else {
        (quote! { #format_expr }, false)
    };

    let expanded = if is_literal_string {
        quote! {
            eprintln!(#processed_format_string_tokens, #(#other_args),*);
        }
    } else {
        quote! {
            eprintln!("{}", #processed_format_string_tokens, #(#other_args),*);
        }
    };

    expanded.into()
}