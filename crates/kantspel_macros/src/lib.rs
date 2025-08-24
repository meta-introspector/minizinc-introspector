use proc_macro::TokenStream;

mod macros;
mod regex_maps;

#[proc_macro]
pub fn kantspel_regex(input: TokenStream) -> TokenStream {
    macros::kantspel_regex::kantspel_regex_impl(input)
}

#[proc_macro_attribute]
pub fn kantspel_transform(attr: TokenStream, item: TokenStream) -> TokenStream {
    macros::kantspel_transform::kantspel_transform_impl(attr, item)
}
