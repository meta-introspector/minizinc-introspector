use proc_macro2::TokenStream as ProcMacro2TokenStream;
use quote::quote;

pub fn append_segment_and_clear(
    current_segment: &mut String,
    result_tokens: &mut ProcMacro2TokenStream,
) {
    if !current_segment.is_empty() {
        result_tokens.extend(quote! { #current_segment });
        current_segment.clear();
    }
}
