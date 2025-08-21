use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemFn;

pub fn poem_function_impl(input_fn: ItemFn) -> TokenStream {
    let fn_name = &input_fn.sig.ident;

    // Generate a helper function that returns the boxed closure
    let _helper_fn_name = quote::format_ident!("__get_fn_{}", fn_name);

    // Define static_name here, outside the expanded quote! block
    let static_name = quote::format_ident!("__REGISTER_FN_{}", fn_name);

    let expanded = quote! {
        use anyhow::Result;
        use std::collections::HashMap;
        use std::sync::LazyLock;

        type PoemFnPtr = Box<dyn Fn(&str, Vec<String>, &mut std::collections::HashMap<String, String>) -> anyhow::Result<(), anyhow::Error> + Send + Sync + 'static>;

        #input_fn

        #[doc(hidden)]
        pub fn #_helper_fn_name() -> Box<dyn Fn(&str, Vec<String>, &mut std::collections::HashMap<String, String>) -> anyhow::Result<(), anyhow::Error> + Send + Sync + 'static> {
            Box::new(|line, captures, fixed_fm| {
                #fn_name(line, captures, fixed_fm)
            })
        }

        pub static #static_name: LazyLock<(String, PoemFnPtr)> = LazyLock::new(|| {
            (stringify!(#fn_name).to_string(), #_helper_fn_name())
        });
    };

    TokenStream::from(expanded)
}