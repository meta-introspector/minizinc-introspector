use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};
use poem_macro_impl::poem_function_impl;

#[proc_macro_attribute]
pub fn poem_function(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    poem_function_impl(input_fn).into() // Converts proc_macro2::TokenStream to proc_macro::TokenStream.
}
