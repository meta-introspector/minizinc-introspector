use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemFn;

pub fn poem_function_impl(input_fn: ItemFn) -> TokenStream {
//    type PoemFnPtr = fn() -> Box<dyn Fn(&str, Vec<String>, &mut std::collections::HashMap<String, String>) -> anyhow::Result<(), anyhow::Error> + Send + Sync + 'static>;

    let fn_name = &input_fn.sig.ident;

    // Generate a helper function that returns the boxed closure
    let _helper_fn_name = quote::format_ident!("__get_fn_{}", fn_name);

    let expanded = quote! {
        use anyhow::Result;
        use std::collections::HashMap;

        #input_fn

        // #[doc(hidden)]
        // pub fn #helper_fn_name() -> Box<dyn Fn(&str, Vec<String>, &mut std::collections::HashMap<String, String>) -> anyhow::Result<(), anyhow::Error> + Send + Sync + 'static> {
        //     Box::new(|line, captures, fixed_fm| {
        //         #fn_name(line, captures, fixed_fm)
        //     })
        // }

        // // Generate a static item that holds the function name and a function pointer to the helper
        // // Removed linkme::distributed_slice to break dependency on poem_yaml_fixer
        // // This static item is now just for demonstration of the generated type
        // pub static __REGISTER_FN_ #fn_name: &'static (String, PoemFnPtr) = &{
        //     (stringify!(#fn_name).to_string(), #helper_fn_name)
        // };
    };

    TokenStream::from(expanded)
}