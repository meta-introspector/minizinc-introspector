//use proc_macro2::TokenStream as ProcMacro2TokenStream;
use quote::quote;
use syn::Expr;
use proc_macro2::TokenStream; // Add this import
use syn::LitStr; // Add this import

pub fn generate_eprintln_tokens(
    processed_format_string_literal: LitStr, // Change type and name
    is_literal_string: bool,
    other_args: Vec<&Expr>,
) -> TokenStream {
    let expanded = if is_literal_string {
        quote! {
            eprintln!(#processed_format_string_literal, #(#other_args),*);
        }
    } else {
        quote! {
            eprintln!("{}", #processed_format_string_literal, #(#other_args),*);
        }
    };

    expanded
}
