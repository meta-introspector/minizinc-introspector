use anyhow::Result;
use std::path::PathBuf;
use syn::NestedMeta;

pub fn extract_patterns_from_rust_file(file_path: &PathBuf) -> Result<Vec<String>> {
    let content = std::fs::read_to_string(file_path)?;
    let syntax_tree = syn::parse_file(&content)?;

    let mut patterns = Vec::new();

    for item in syntax_tree.items {
        if let syn::Item::Fn(item_fn) = item {
            for attr in item_fn.attrs {
                if attr.path().is_ident("poem_function") {
                    if let syn::Meta::List(meta_list) = attr.meta {
                        for nested_meta in meta_list.nested.iter() {
                            if let syn::Meta::NameValue(name_value) = nested_meta { // Directly match NameValue
                                if name_value.path.is_ident("pattern") {
                                    if let syn::Expr::Lit(expr_lit) = name_value.value {
                                        if let syn::Lit::Str(lit_str) = expr_lit.lit {
                                        patterns.push(lit_str.value());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(patterns)
}
