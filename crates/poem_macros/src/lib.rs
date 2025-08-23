use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};
use proc_macro2::TokenStream as ProcMacro2TokenStream;
use poem_macro_impl::{poem_function_impl, poem_header_impl};

#[proc_macro_attribute]
pub fn poem_function(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let attr_tokens = ProcMacro2TokenStream::from(attr);
    poem_function_impl(attr_tokens, input_fn).into()
}

#[proc_macro]
pub fn poem_header(_input: TokenStream) -> TokenStream {
    poem_header_impl().into()
}
