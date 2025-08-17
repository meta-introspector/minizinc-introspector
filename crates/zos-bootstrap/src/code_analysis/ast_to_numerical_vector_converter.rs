use syn::{visit::{self, Visit}, File, Item, Ident, Lit, Expr, FnArg, PatType};

use crate::code_analysis::numerical_vector_generator::{get_prime_for_vocabulary, compose_numerical_vector};

#[derive(Debug, Clone)]
pub enum AstElement {
    Crate(String),
    Module(String),
    Function(String),
    FunctionArg(String),
    FunctionArgType(String),
    Struct(String),
    Enum(String),
    Const(String),
    Static(String),
    Literal(String),
    Identifier(String),
    // Add more AST elements as needed
}

impl AstElement {
    pub fn to_vocabulary_string(&self) -> String {
        match self {
            AstElement::Crate(s) => format!("crate_{}", s),
            AstElement::Module(s) => format!("module_{}", s),
            AstElement::Function(s) => format!("fn_{}", s),
            AstElement::FunctionArg(s) => format!("fn_arg_{}", s),
            AstElement::FunctionArgType(s) => format!("fn_arg_type_{}", s),
            AstElement::Struct(s) => format!("struct_{}", s),
            AstElement::Enum(s) => format!("enum_{}", s),
            AstElement::Const(s) => format!("const_{}", s),
            AstElement::Static(s) => format!("static_{}", s),
            AstElement::Literal(s) => format!("literal_{}", s),
            AstElement::Identifier(s) => format!("ident_{}", s),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AstNumericalVector {
    pub element: AstElement,
    pub numerical_vector: u128,
}

pub struct AstToNumericalVectorConverter {
    pub ast_numerical_vectors: Vec<AstNumericalVector>,
    current_crate_name: String,
    current_module_path: Vec<String>,
    current_function_name: Option<String>,
}

impl AstToNumericalVectorConverter {
    pub fn new(crate_name: String) -> Self {
        AstToNumericalVectorConverter {
            ast_numerical_vectors: Vec::new(),
            current_crate_name: crate_name,
            current_module_path: Vec::new(),
            current_function_name: None,
        }
    }

    fn add_element(&mut self, element: AstElement) {
        let vocab_string = element.to_vocabulary_string();
        let prime = get_prime_for_vocabulary(&vocab_string);
        let numerical_vector = compose_numerical_vector(&[prime]); // For now, single prime per element
        self.ast_numerical_vectors.push(AstNumericalVector {
            element,
            numerical_vector,
        });
    }

    fn extract_literal_value(lit: &Lit) -> String {
        match lit {
            Lit::Str(lit_str) => lit_str.value(),
            Lit::ByteStr(lit_byte_str) => String::from_utf8_lossy(&lit_byte_str.value()).to_string(),
            Lit::Byte(lit_byte) => lit_byte.value().to_string(),
            Lit::Char(lit_char) => lit_char.value().to_string(),
            Lit::Int(lit_int) => lit_int.to_string(),
            Lit::Float(lit_float) => lit_float.to_string(),
            Lit::Bool(lit_bool) => lit_bool.value.to_string(),
            _ => "[Unsupported literal type]".to_string(),
        }
    }
}

impl<'ast> Visit<'ast> for AstToNumericalVectorConverter {
    fn visit_file(&mut self, i: &'ast File) {
        self.add_element(AstElement::Crate(self.current_crate_name.clone()));
        visit::visit_file(self, i);
    }

    fn visit_item(&mut self, i: &'ast Item) {
        match i {
            Item::Mod(item_mod) => {
                self.current_module_path.push(item_mod.ident.to_string());
                self.add_element(AstElement::Module(item_mod.ident.to_string()));
                visit::visit_item_mod(self, item_mod);
                self.current_module_path.pop();
            }
            Item::Fn(item_fn) => {
                let old_function_name = self.current_function_name.take();
                self.current_function_name = Some(item_fn.sig.ident.to_string());
                self.add_element(AstElement::Function(item_fn.sig.ident.to_string()));

                // Extract function arguments and their types
                for input in &item_fn.sig.inputs {
                    match input {
                        FnArg::Receiver(_) => {
                            // `self` argument, can be ignored or handled specifically
                            self.add_element(AstElement::FunctionArg("self".to_string()));
                            self.add_element(AstElement::FunctionArgType("self_type".to_string())); // Placeholder for self type
                        },
                        FnArg::Typed(PatType { pat, ty, .. }) => {
                            if let Pat::Ident(pat_ident) = &**pat {
                                self.add_element(AstElement::FunctionArg(pat_ident.ident.to_string()));
                            }
                            // Extract type string
                            self.add_element(AstElement::FunctionArgType(ty.to_token_stream().to_string()));
                        }
                    }
                }

                visit::visit_item_fn(self, item_fn);
                self.current_function_name = old_function_name;
            }
            Item::Struct(item_struct) => {
                self.add_element(AstElement::Struct(item_struct.ident.to_string()));
                visit::visit_item_struct(self, item_struct);
            }
            Item::Enum(item_enum) => {
                self.add_element(AstElement::Enum(item_enum.ident.to_string()));
                visit::visit_item_enum(self, item_enum);
            }
            Item::Const(item_const) => {
                self.add_element(AstElement::Const(item_const.ident.to_string()));
                if let Expr::Lit(expr_lit) = &*item_const.expr {
                    self.add_element(AstElement::Literal(Self::extract_literal_value(&expr_lit.lit)));
                }
                visit::visit_item_const(self, item_const);
            }
            Item::Static(item_static) => {
                self.add_element(AstElement::Static(item_static.ident.to_string()));
                if let Expr::Lit(expr_lit) = &*item_static.expr {
                    self.add_element(AstElement::Literal(Self::extract_literal_value(&expr_lit.lit)));
                }
                visit::visit_item_static(self, item_static);
            }
            _ => visit::visit_item(self, i),
        }
    }

    fn visit_expr(&mut self, i: &'ast Expr) {
        if let Expr::Lit(expr_lit) = i {
            self.add_element(AstElement::Literal(Self::extract_literal_value(&expr_lit.lit)));
        }
        visit::visit_expr(self, i);
    }

    fn visit_ident(&mut self, i: &'ast Ident) {
        self.add_element(AstElement::Identifier(i.to_string()));
        visit::visit_ident(self, i);
    }

    // Add more visit methods for other AST nodes as needed
}

pub fn convert_ast_to_numerical_vectors(file: &File, crate_name: String) -> Vec<AstNumericalVector> {
    let mut converter = AstToNumericalVectorConverter::new(crate_name);
    converter.visit_file(file);
    converter.ast_numerical_vectors
}
