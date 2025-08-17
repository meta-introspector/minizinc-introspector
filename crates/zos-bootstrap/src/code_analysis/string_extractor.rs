use syn::{Lit, visit::{self, Visit}, Item, ImplItem, FnArg, Pat, PatType};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct ExtractedString {
    pub crate_name: String,
    pub module_path: Vec<String>,
    pub function_name: Option<String>,
    pub variable_name: Option<String>,
    pub string_value: String,
}

pub struct StringExtractor {
    pub extracted_strings: Vec<ExtractedString>,
    current_crate: String,
    current_module_path: Vec<String>,
    current_function_name: Option<String>,
}

impl StringExtractor {
    pub fn new(crate_name: String) -> Self {
        StringExtractor {
            extracted_strings: Vec::new(),
            current_crate: crate_name,
            current_module_path: Vec::new(),
            current_function_name: None,
        }
    }
}

impl<'ast> Visit<'ast> for StringExtractor {
    fn visit_item(&mut self, i: &'ast Item) {
        match i {
            Item::Mod(item_mod) => {
                self.current_module_path.push(item_mod.ident.to_string());
                visit::visit_item_mod(self, item_mod);
                self.current_module_path.pop();
            }
            Item::Fn(item_fn) => {
                let old_function_name = self.current_function_name.take();
                self.current_function_name = Some(item_fn.sig.ident.to_string());
                visit::visit_item_fn(self, item_fn);
                self.current_function_name = old_function_name;
            }
            Item::Const(item_const) => {
                // Handle const items
                if let syn::Expr::Lit(expr_lit) = &*item_const.expr {
                    if let Lit::Str(lit_str) = &expr_lit.lit {
                    self.extracted_strings.push(ExtractedString {
                        crate_name: self.current_crate.clone(),
                        module_path: self.current_module_path.clone(),
                        function_name: self.current_function_name.clone(),
                        variable_name: Some(item_const.ident.to_string()),
                        string_value: lit_str.value(),
                    });
                }
            }
            visit::visit_item_const(self, item_const);
            }
            Item::Static(item_static) => {
                // Handle static items
                if let syn::Expr::Lit(expr_lit) = &*item_static.expr {
                    if let Lit::Str(lit_str) = &expr_lit.lit {
                    self.extracted_strings.push(ExtractedString {
                        crate_name: self.current_crate.clone(),
                        module_path: self.current_module_path.clone(),
                        function_name: self.current_function_name.clone(),
                        variable_name: Some(item_static.ident.to_string()),
                        string_value: lit_str.value(),
                    });
                }
            }
            visit::visit_item_static(self, item_static);
            }
            _ => visit::visit_item(self, i),
        }
    }

    fn visit_expr(&mut self, i: &'ast syn::Expr) {
        if let syn::Expr::Lit(expr_lit) = i {
            if let Lit::Str(lit_str) = &expr_lit.lit {
                // Only capture if not already captured by Item::Const or Item::Static
                // This is a simplification; a more robust solution would track assignments
                // For now, we'll capture all string literals in expressions
                self.extracted_strings.push(ExtractedString {
                    crate_name: self.current_crate.clone(),
                    module_path: self.current_module_path.clone(),
                    function_name: self.current_function_name.clone(),
                    variable_name: None, // Not a const/static variable
                    string_value: lit_str.value(),
                });
            }
        }
        visit::visit_expr(self, i);
    }

    // Handle string literals in function arguments (e.g., println!, format!)
    fn visit_fn_arg(&mut self, i: &'ast FnArg) {
        if let FnArg::Typed(PatType { pat, ty, .. }) = i {
            if let Pat::Ident(_pat_ident) = pat.as_ref() {
                // Check if the type is a string literal or a reference to one
                // This is a heuristic and might need refinement
                if let syn::Type::Reference(type_ref) = ty.as_ref() {
                    if let syn::Type::Path(type_path) = type_ref.elem.as_ref() {
                        if type_path.path.segments.last().map_or(false, |s| s.ident == "str" || s.ident == "String") {
                            // We found a string argument, but we need its value.
                            // This requires more advanced analysis (e.g., data flow)
                            // For now, we'll just note its presence.
                            // A more complete solution would involve analyzing the call site.
                        }
                    }
                }
            }
        }
        visit::visit_fn_arg(self, i);
    }

    fn visit_impl_item(&mut self, i: &'ast ImplItem) {
        if let ImplItem::Fn(impl_item_fn) = i {
            let old_function_name = self.current_function_name.take();
            self.current_function_name = Some(impl_item_fn.sig.ident.to_string());
            visit::visit_impl_item_fn(self, impl_item_fn);
            self.current_function_name = old_function_name;
        } else {
            visit::visit_impl_item(self, i);
        }
    }
}

pub fn extract_strings_from_file(file_path: &Path, crate_name: String) -> Result<Vec<ExtractedString>, Box<dyn std::error::Error>> {
    let code = fs::read_to_string(file_path)?;
    let syntax = syn::parse_file(&code)?;

    let mut extractor = StringExtractor::new(crate_name);
    extractor.visit_file(&syntax);

    Ok(extractor.extracted_strings)
}