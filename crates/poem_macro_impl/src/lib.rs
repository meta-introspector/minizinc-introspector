use proc_macro2::TokenStream as ProcMacro2TokenStream;
use quote::quote;
use syn::{ItemFn, LitStr};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;

 // Import PoemFunctionEntry

// Struct to parse the attributes for #[poem_function]
struct PoemFunctionAttrs {
    name: Option<LitStr>,
    pattern: Option<LitStr>,
    title: Option<LitStr>,
    summary: Option<LitStr>,
    keywords: Option<LitStr>,
    emojis: Option<LitStr>,
    art_generator_instructions: Option<LitStr>,
    pending_meme_description: Option<LitStr>,
}

impl Parse for PoemFunctionAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut name = None;
        let mut pattern = None;
        let mut title = None;
        let mut summary = None;
        let mut keywords = None;
        let mut emojis = None;
        let mut art_generator_instructions = None;
        let mut pending_meme_description = None;

        let parsed_attrs = Punctuated::<syn::Meta, Comma>::parse_terminated(input)?;
        for meta in parsed_attrs {
            match meta {
                syn::Meta::NameValue(nv) => {
                    if nv.path.is_ident("name") {
                        if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(s), .. }) = nv.value {
                            name = Some(s);
                        }
                    }
                    else if nv.path.is_ident("pattern") {
                        if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(s), .. }) = nv.value {
                            pattern = Some(s);
                        }
                    }
                    else if nv.path.is_ident("title") {
                        if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(s), .. }) = nv.value {
                            title = Some(s);
                        }
                    }
                    else if nv.path.is_ident("summary") {
                        if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(s), .. }) = nv.value {
                            summary = Some(s);
                        }
                    }
                    else if nv.path.is_ident("keywords") {
                        if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(s), .. }) = nv.value {
                            keywords = Some(s);
                        }
                    }
                    else if nv.path.is_ident("emojis") {
                        if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(s), .. }) = nv.value {
                            emojis = Some(s);
                        }
                    }
                    else if nv.path.is_ident("art_generator_instructions") {
                        if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(s), .. }) = nv.value {
                            art_generator_instructions = Some(s);
                        }
                    }
                    else if nv.path.is_ident("pending_meme_description") {
                        if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(s), .. }) = nv.value {
                            pending_meme_description = Some(s);
                        }
                    }
                }
                _ => return Err(input.error("unsupported attribute")),
            }
        }

        Ok(PoemFunctionAttrs {
            name,
            pattern,
            title,
            summary,
            keywords,
            emojis,
            art_generator_instructions,
            pending_meme_description,
        })
    }
}

#[allow(non_upper_case_globals)]
pub fn poem_function_impl(attr: ProcMacro2TokenStream, input_fn: ItemFn) -> ProcMacro2TokenStream {
    let fn_name = &input_fn.sig.ident;

    // Parse the attributes
    let attrs = syn::parse::Parser::parse2(PoemFunctionAttrs::parse, attr).unwrap();

    let default_name = fn_name.to_string();
    let default_pattern = format!("^{fn_name}$"); // Default pattern based on function name

    let name_lit = attrs.name.unwrap_or_else(|| LitStr::new(&default_name, fn_name.span()));
    let pattern_lit = attrs.pattern.unwrap_or_else(|| LitStr::new(&default_pattern, fn_name.span()));

    let title_lit = attrs.title.map(|s| quote! { Some(#s.to_string()) }).unwrap_or_else(|| quote! { None });
    let summary_lit = attrs.summary.map(|s| quote! { Some(#s.to_string()) }).unwrap_or_else(|| quote! { None });
    let keywords_lit = attrs.keywords.map(|s| quote! { Some(#s.to_string()) }).unwrap_or_else(|| quote! { None });
    let emojis_lit = attrs.emojis.map(|s| quote! { Some(#s.to_string()) }).unwrap_or_else(|| quote! { None });
    let art_generator_instructions_lit = attrs.art_generator_instructions.map(|s| quote! { Some(#s.to_string()) }).unwrap_or_else(|| quote! { None });
    let pending_meme_description_lit = attrs.pending_meme_description.map(|s| quote! { Some(#s.to_string()) }).unwrap_or_else(|| quote! { None });


    // Generate a helper function that returns the boxed closure
    let helper_fn_name = quote::format_ident!("__get_fn_{}", fn_name);

    // Define static_name here, outside the expanded quote! block
    let static_name = quote::format_ident!("__REGISTER_FN_{}", fn_name.to_string().to_uppercase());

    let expanded = quote! {
        #input_fn

        #[doc(hidden)]
        pub fn #helper_fn_name() -> poem_traits::CallbackFn { // Use CallbackFn type alias
            Box::new(|line, captures, fixed_fm| {
                #fn_name(line, captures, fixed_fm)
            })
        }

        pub static #static_name: std::sync::LazyLock<poem_traits::PoemFunctionEntry> = std::sync::LazyLock::new(|| {
            (
                poem_traits::PoemFunctionMetadata {
                    regex_entry: poem_traits::RegexEntry {
                        name: #name_lit.to_string(),
                        pattern: #pattern_lit.to_string(),
                        callback_function: stringify!(#fn_name).to_string(),
                    },
                    title: #title_lit,
                    summary: #summary_lit,
                    keywords: #keywords_lit,
                    emojis: #emojis_lit,
                    art_generator_instructions: #art_generator_instructions_lit,
                    pending_meme_description: #pending_meme_description_lit,
                },
                #helper_fn_name()
            )
        });
    };

    expanded
}

pub fn poem_header_impl() -> ProcMacro2TokenStream {
    quote! {
        use std::collections::HashMap;
        use anyhow::Result;
        use linkme::distributed_slice;
        use poem_traits::{PoemFrontMatterTrait, Meme, RegexEntry, RegexConfig, CallbackFn, PoemFunctionMetadata, PoemFunctionEntry, FunctionRegistry}; // Import FunctionRegistry

        #[distributed_slice]
        #[allow(non_upper_case_globals)] // Suppress warnings for macro-generated static variables
        pub static FUNCTIONS: [&'static PoemFunctionEntry]; // Use PoemFunctionEntry

        pub fn create_function_registry() -> FunctionRegistry { // Use FunctionRegistry as return type
            let mut registry = HashMap::new();
            for entry in FUNCTIONS { // Iterate over PoemFunctionEntry
                registry.insert(entry.0.regex_entry.callback_function.clone(), (*entry).clone()); // Insert the whole PoemFunctionEntry
            }
            registry
        }

        pub fn get_default_regex_config() -> RegexConfig {
            let mut regexes = Vec::new();
            for (metadata, _callback_fn) in FUNCTIONS { // Iterate over PoemFunctionEntry
                regexes.push(metadata.regex_entry.clone()); // Clone the RegexEntry from metadata
            }
            RegexConfig { regexes }
        }
    }
}

pub fn poem_header(_input: ProcMacro2TokenStream) -> ProcMacro2TokenStream {
    poem_header_impl()
}