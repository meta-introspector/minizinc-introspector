use proc_macro::TokenStream;
use syn::{parse_macro_input,
	  //LitStr,
	  Expr, ExprLit, punctuated::Punctuated, token::Comma};
use quote::quote;
use proc_macro2::TokenStream as ProcMacro2TokenStream;

//include!(concat!(env!("OUT_DIR"), "/emojis.rs"));

struct CommaSeparatedExprs {
    exprs: Punctuated<Expr, Comma>,
}

impl syn::parse::Parse for CommaSeparatedExprs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(CommaSeparatedExprs {
            exprs: Punctuated::parse_terminated(input)?,
        })
    }
}

#[proc_macro]
pub fn gemini_eprintln(input: TokenStream) -> TokenStream {
    let input_args = parse_macro_input!(input as CommaSeparatedExprs).exprs;

    if input_args.is_empty() {
        return quote! { eprintln!(); }.into();
    }

    let format_expr = &input_args[0];
    let other_args = input_args.iter().skip(1).collect::<Vec<_>>();

    let final_format_string_expr = if let Expr::Lit(ExprLit { lit: syn::Lit::Str(s), .. }) = format_expr {
        let original_string = s.value();

        let mut result_tokens = ProcMacro2TokenStream::new();
        let mut current_segment = String::new();

        let mut chars = original_string.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                '\\'=> {
                    if let Some('n') = chars.peek() {
                        chars.next(); // consume 'n'
                        if !current_segment.is_empty() {
                            result_tokens.extend(quote! { #current_segment });
                            current_segment.clear();
                        }
                        result_tokens.extend(quote! { *EMOJIS.get("return").unwrap_or(&"⏎") });
                    } else {
                        current_segment.push(c);
                    }
                },
                '{' => {
                    if let Some('{') = chars.peek() {
                        chars.next(); // consume '{'
                        if let Some('}') = chars.peek() {
                            chars.next(); // consume '}'
                            if let Some('}') = chars.peek() {
                                chars.next(); // consume '}'
                                if !current_segment.is_empty() {
                                    result_tokens.extend(quote! { #current_segment });
                                    current_segment.clear();
                                }
                                result_tokens.extend(quote! { *EMOJIS.get("brick").unwrap_or(&"🧱") });
                            } else {
                                current_segment.push('{');
                                current_segment.push('{');
                                current_segment.push('}');
                            }
                        } else {
                            current_segment.push('{');
                            current_segment.push('{');
                        }
                    } else if let Some('}') = chars.peek() {
                        chars.next(); // consume '}'
                        if !current_segment.is_empty() {
                            result_tokens.extend(quote! { #current_segment });
                            current_segment.clear();
                        }
                        result_tokens.extend(quote! { *EMOJIS.get("sparkles").unwrap_or(&"✨") });
                    } else {
                        current_segment.push(c);
                    }
                },
                _ => current_segment.push(c),
            }
        }
        if !current_segment.is_empty() {
            result_tokens.extend(quote! { #current_segment });
        }

        // Generate a `format!` call that produces the final string
        quote! { format!(#result_tokens) }

    } else {
        // If the first argument is not a string literal, use it as is.
        quote! { #format_expr }
    };

    let expanded = quote! {
        eprintln!("{}", #final_format_string_expr, #(#other_args),*);
    };

    expanded.into()
}
