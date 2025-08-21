use proc_macro::TokenStream;
use quote::quote;
//use proc_macro::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn poem_function(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;

    let expanded = quote! {
        #input_fn

        // Generate a static item that holds the function name and a function pointer to the original function.
        // This static item is placed into a distributed slice using linkme.
        #[linkme::distributed_slice(poem_yaml_fixer::functions::create_function_registry::FUNCTIONS)]
        static __REGISTER_FN_ #fn_name: &'static (String, fn(&str, Vec<String>, &mut poem_yaml_fixer::functions::types::FixedFrontMatter) -> anyhow::Result<(), anyhow::Error>)
            = &{
            (stringify!(#fn_name).to_string(), #fn_name)
        };
    };

    TokenStream::from(expanded)
}
