use proc_macro::TokenStream;
use syn::{parse_macro_input, visit_mut::{self, VisitMut}, LitStr};
use quote::quote;
use kantspel_lib::{BACKSLASH, OPEN_CURLY_BRACE, CLOSE_CURLY_BRACE};

// Define the KantspelTransformer struct
pub struct KantspelTransformer;

// Implement VisitMut for KantspelTransformer to traverse and modify the AST
impl VisitMut for KantspelTransformer {
    fn visit_expr_lit_mut(&mut self, i: &mut syn::ExprLit) {
        if let syn::Lit::Str(lit_str) = &mut i.lit {
            let original_string = lit_str.value();
            let mut modified_string = String::new();

            let mut chars = original_string.chars().peekable();
            while let Some(c) = chars.next() {
                match c {
                    BACKSLASH => {
                        // Replace with BACKSLASH constant
                        modified_string.push_str(&BACKSLASH.to_string());
                    },
                    OPEN_CURLY_BRACE => {
                        // Check for {{ 
                        if let Some(&OPEN_CURLY_BRACE) = chars.peek() {
                            chars.next(); // consume second '{'
                            modified_string.push_str(&OPEN_CURLY_BRACE.to_string());
                            modified_string.push_str(&OPEN_CURLY_BRACE.to_string());
                        } else {
                            modified_string.push_str(&OPEN_CURLY_BRACE.to_string());
                        }
                    },
                    CLOSE_CURLY_BRACE => {
                        // Check for }}
                        if let Some(&CLOSE_CURLY_BRACE) = chars.peek() {
                            chars.next(); // consume second '}'
                            modified_string.push_str(&CLOSE_CURLY_BRACE.to_string());
                            modified_string.push_str(&CLOSE_CURLY_BRACE.to_string());
                        } else {
                            modified_string.push_str(&CLOSE_CURLY_BRACE.to_string());
                        }
                    },
                    _ => modified_string.push(c),
                }
            }
            *lit_str = LitStr::new(&modified_string, lit_str.span());
        }
        // Continue visiting other parts of the literal
        visit_mut::visit_expr_lit_mut(self, i);
    }
}

pub fn kantspel_transform_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as syn::File); // Parse the entire file
    let mut transformer = KantspelTransformer;
    transformer.visit_file_mut(&mut ast); // Visit and transform the file's AST
    quote! { #ast }.into() // Convert the modified AST back to TokenStream
}
