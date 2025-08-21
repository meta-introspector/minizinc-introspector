use proc_macro2::TokenStream;
use quote::quote;
//use syn::{parse_macro_input, ItemFn};

pub fn _poem_function_impl(_input_fn:&str, fn_name: &str, _item: TokenStream) -> TokenStream {
//    let input_fn = parse_macro_input!(item as ItemFn);
    //let fn_name = &input_fn.sig.ident;

    // Generate a helper function that returns the boxed closure
    let helper_fn_name = quote::format_ident!("__get_fn_{}", fn_name);

    let expanded = quote! {
        input_fn

        #[doc(hidden)]
        pub fn #helper_fn_name() -> Box<dyn Fn(&str, Vec<String>, &mut poem_yaml_fixer::functions::types::FixedFrontMatter) -> anyhow::Result<(), anyhow::Error> + Send + Sync + 'static> {
            Box::new(|line, captures, fixed_fm| {
                #fn_name(line, captures, fixed_fm)
            })
        }

        // Generate a static item that holds the function name and a function pointer to the helper
        #[linkme::distributed_slice(poem_yaml_fixer::functions::create_function_registry::FUNCTIONS)]
        static __REGISTER_FN_ #fn_name: &'static (String, fn() -> Box<dyn Fn(&str, Vec<String>, &mut poem_yaml_fixer::functions::types::FixedFrontMatter) -> anyhow::Result<(), anyhow::Error> + Send + Sync + 'static>)
            = &{
            (stringify!(#fn_name).to_string(), #helper_fn_name)
        };
    };

    TokenStream::from(expanded)
}
