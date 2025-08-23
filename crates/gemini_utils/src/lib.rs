use proc_macro::TokenStream;
use syn::{parse_macro_input, Expr, ExprLit};
use quote::quote;
use proc_macro2::TokenStream as ProcMacro2TokenStream;

use kantspel_lib::*;
mod macro_parser;
mod string_processor;

use macro_parser::comma_separated_exprs::CommaSeparatedExprs;
use string_processor::{process_char_for_emojis, append_segment_and_clear};



#[proc_macro]
pub fn gemini_eprintln(input: TokenStream) -> TokenStream {
    let input_args = parse_macro_input!(input as CommaSeparatedExprs).exprs;

    if input_args.is_empty() {
        return quote! { eprintln!(); }.into();
    }

    let format_expr = &input_args[0];
    let other_args = input_args.iter().skip(1).collect::<Vec<_>>();

    let final_format_string_expr = if let Expr::Lit(ExprLit { lit: syn::Lit::Str(s), .. }) = format_expr {
        let original_string = s.value();

        let mut result_tokens = ProcMacro2TokenStream::new();
        let mut current_segment = String::new();

        let mut chars = original_string.chars().peekable();
        while let Some(c) = chars.next() {
            process_char_for_emojis(c, &mut chars, &mut current_segment, &mut result_tokens);
        }
        append_segment_and_clear(&mut current_segment, &mut result_tokens);

        // Generate a `format!` call that produces the final string
        quote! { format!(#result_tokens) }

    } else {
        // If the first argument is not a string literal, use it as is.
        quote! { #format_expr }
    };

    let expanded = quote! {
        eprintln!("{}", #final_format_string_expr, #(#other_args),*);
    };

    expanded.into()
}