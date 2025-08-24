use proc_macro::TokenStream;
use syn::{parse_macro_input, LitStr, visit_mut::{self, VisitMut}};
use quote::quote;
use lazy_static::lazy_static;
use std::collections::HashMap;

// Import kantspel constants
use kantspel_lib::{BACKSLASH, OPEN_CURLY_BRACE, CLOSE_CURLY_BRACE};

lazy_static! {
    static ref REGEX_EMOJIS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        // General
        map.insert("💨", r"\u{1b}"); // Escape character
        map.insert("🤸", r"\s"); // Whitespace
        map.insert("🔢", r"\d"); // Digit
        map.insert("✍️", r"\w"); // Word character
        map.insert(".", ".");   // Any character
        map.insert("❓", ".");   // Any character (alternative)

        // Quantifiers
        map.insert("*", "*");   // Zero or more
        map.insert("+", "+");   // One or more
        map.insert("?", "?");   // Zero or one

        // Anchors
        map.insert("^", "^");   // Start of string
        map.insert("$", "$");   // End of string

        // Grouping and Alternation
        map.insert("(", "(");
        map.insert(")", ")");
        map.insert("[", "[");
        map.insert("]", "]");
        map.insert("{", "{");
        map.insert("}", "}");
        map.insert("|", "|");

        // Escaped characters
        map.insert(r"\\", r"\\"); // Literal backslash

        // Special characters often needing escape
        map.insert(r"\.", r"\.");
        map.insert(r"\*", r"\*");
        map.insert(r"\+", r"\+");
        map.insert(r"\?", r"\?");
        map.insert(r"\(", r"\");
        map.insert(r"\)", r"\");
        map.insert(r"\[", r"\");
        map.insert(r"\]", r"\");
        map.insert(r"{{", r"\");
        map.insert(r"}}", r"\");
        map.insert(r"\^", r"\");
        map.insert(r"\$", r"\");
        map.insert(r"\|", r"\");

        map
    };
}

#[proc_macro]
pub fn kantspel_regex(input: TokenStream) -> TokenStream {
    let lit_str = parse_macro_input!(input as LitStr);
    let original_string = lit_str.value();
    let mut modified_string = original_string.clone();

    for (emoji, replacement) in REGEX_EMOJIS.iter() {
        modified_string = modified_string.replace(emoji, replacement);
    }

    let output = quote! { #modified_string };
    output.into()
}

// Define the KantspelTransformer struct
struct KantspelTransformer;

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

// Define the #[kantspel_transform] attribute macro
#[proc_macro_attribute]
pub fn kantspel_transform(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as syn::File); // Parse the entire file
    let mut transformer = KantspelTransformer;
    transformer.visit_file_mut(&mut ast); // Visit and transform the file's AST
    quote! { #ast }.into() // Convert the modified AST back to TokenStream
}

