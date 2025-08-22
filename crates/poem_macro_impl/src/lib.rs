use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemFn;
//use poem_traits::PoemFrontMatterTrait; // Changed import

pub fn poem_function_impl(input_fn: ItemFn) -> TokenStream {
    let fn_name = &input_fn.sig.ident;

    // Generate a helper function that returns the boxed closure
    let _helper_fn_name = quote::format_ident!("__get_fn_{}", fn_name);

    // Define static_name here, outside the expanded quote! block
    let static_name = quote::format_ident!("__REGISTER_FN_{}", fn_name);

    let expanded = quote! {
        #input_fn

        #[doc(hidden)]
        pub fn #_helper_fn_name() -> Box<dyn Fn(&str, Vec<String>, &mut dyn PoemFrontMatterTrait) -> anyhow::Result<(), anyhow::Error> + Send + Sync + 'static> {
            Box::new(|line, captures, fixed_fm| {
                #fn_name(line, captures, fixed_fm)
            })
        }

        pub static #static_name: std::sync::LazyLock<(String, Box<dyn Fn(&str, Vec<String>, &mut dyn PoemFrontMatterTrait) -> anyhow::Result<(), anyhow::Error> + Send + Sync + 'static>)> = std::sync::LazyLock::new(|| {
            (stringify!(#fn_name).to_string(), #_helper_fn_name())
        });
    };

    TokenStream::from(expanded)
}

pub fn poem_header_impl() -> TokenStream {
    quote! {
        use std::collections::HashMap;
        use anyhow::Result;
        use once_cell::sync::LazyLock;
        use linkme::distributed_slice;
        use poem_traits::{PoemFrontMatterTrait, Meme}; // Import Meme from poem_traits

        type PoemFnPtr = Box<dyn Fn(&str, Vec<String>, &mut dyn PoemFrontMatterTrait) -> anyhow::Result<(), anyhow::Error> + Send + Sync + 'static>;

        #[distributed_slice]
        pub static FUNCTIONS: [&'static (String, fn() -> PoemFnPtr)];

        pub fn create_function_registry() -> &'static HashMap<String, PoemFnPtr> {
            static REGISTRY: LazyLock<HashMap<String, PoemFnPtr>> = LazyLock::new(|| {
                let mut registry = HashMap::new();
                for (name, callback_fn_getter) in FUNCTIONS {
                    registry.insert(name.clone(), callback_fn_getter());
                }
                registry
            });
            &REGISTRY
        }
    }
}
