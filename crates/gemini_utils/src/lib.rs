use proc_macro::TokenStream;
use syn::{parse_macro_input, //Expr, ExprLit,
	  LitStr};
use quote::{quote,
	    //ToTokens
};
//use proc_macro2::TokenStream as ProcMacro2TokenStream;
//use lazy_static::lazy_static; // Add this import
//use std::collections::HashMap; // Add this import
//use proc_macro2::Span; // Add this import

//use kantspel_lib::*;
mod macro_parser;
mod string_processor;
mod token_generator; // Add this module

//use macro_parser::comma_separated_exprs::CommaSeparatedExprs;
use string_processor::{process_char_for_emojis, EMOJIS};
//use token_generator::generate_eprintln_tokens::generate_eprintln_tokens; // Add this use statement

// Define EMOJIS here
use crate::macro_parser::named_args_input::Input;

#[proc_macro]
pub fn gemini_eprintln(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Input);

    let format_string_value = input.format_string.value();
    let format_string_span = input.format_string.span();

    // Translate named placeholders (:key:) to standard {} placeholders
    // This is a simplified approach; a more robust solution might use regex
    let mut translated_format_string = String::new();
    let mut in_placeholder = false;
    for c in format_string_value.chars() {
        if c == ':' && !in_placeholder {
            in_placeholder = true;
            translated_format_string.push('{');
        } else if c == ':' && in_placeholder {
            in_placeholder = false;
            translated_format_string.push('}');
        } else {
            translated_format_string.push(c);
        }
    }

    let mut current_segment = String::new();
    let mut chars = translated_format_string.chars().peekable(); // Use translated_format_string
    let mut context = string_processor::processing_context::ProcessingContext {
        chars: &mut chars,
        current_segment: &mut current_segment,
        emojis: &EMOJIS,
    };
    while let Some(c) = context.chars.next() {
        process_char_for_emojis(c, &mut context);
    }
    let processed_format_string = LitStr::new(&current_segment, format_string_span);

    let named_args = input.named_args;

    // Generate the eprintln! call
    let expanded = if let Some(args) = named_args {
        quote! {
            eprintln!(#processed_format_string, #args);
        }
    } else {
        quote! {
            eprintln!(#processed_format_string);
        }
    };

    expanded.into()
}
