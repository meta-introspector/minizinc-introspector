use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemFn;

pub fn poem_function_impl(input_fn: ItemFn) -> TokenStream {
    let fn_name = &input_fn.sig.ident;

    // Generate a helper function that returns the boxed closure
    let helper_fn_name = quote::format_ident!("__get_fn_{}", fn_name);

    let expanded = quote! {
        #input_fn

        #[doc(hidden)]
        pub fn #helper_fn_name() -> Box<dyn Fn(&str, Vec<String>, &mut std::collections::HashMap<String, String>) -> anyhow::Result<(), anyhow::Error> + Send + Sync + 'static> {
            Box::new(|line, captures, fixed_fm| {
                #fn_name(line, captures, fixed_fm)
            })
        }

        // Generate a static item that holds the function name and a function pointer to the helper
        // Removed linkme::distributed_slice to break dependency on poem_yaml_fixer
        // This static item is now just for demonstration of the generated type
        static __REGISTER_FN_ #fn_name: &'static (String, fn() -> Box<dyn Fn(&str, Vec<String>, &mut std::collections::HashMap<String, String>) -> anyhow::Result<(), anyhow::Error> + Send + Sync + 'static>)
            = &{
            (stringify!(#fn_name).to_string(), #helper_fn_name)
        };
    };

    TokenStream::from(expanded)
}