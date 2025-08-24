use proc_macro::TokenStream;
use syn::{parse_macro_input, LitStr};
use quote::quote;
use crate::regex_maps::{REGEX_EMOJIS, REGEX_ALIASES};

pub fn kantspel_regex_impl(input: TokenStream) -> TokenStream {
    let lit_str = parse_macro_input!(input as LitStr);
    let mut modified_string = lit_str.value();

    for (emoji, replacement) in REGEX_EMOJIS.iter() {
        modified_string = modified_string.replace(emoji, replacement);
    }

    for (alias, replacement) in REGEX_ALIASES.iter() {
        let aliased = format!("::{alias}::");
        modified_string = modified_string.replace(&aliased, replacement);
    }

    let output = quote! { #modified_string };
    output.into()
}
