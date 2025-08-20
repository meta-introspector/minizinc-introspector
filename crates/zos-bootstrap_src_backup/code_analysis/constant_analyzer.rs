use std::{fs, path::Path};
use syn::{Item, Expr, Stmt};
use std::collections::HashMap;
use crate::utils::error::Result;
use crate::utils::paths;

pub struct ConstantAnalyzer {
    constant_counts: HashMap<String, usize>,
}

impl ConstantAnalyzer {
    pub fn new() -> Result<Self> {
        let mut constant_counts = HashMap::new();

        let project_root = paths::resolve_project_root()?;

        let build_constants_path = project_root.join("crates/zos-bootstrap/src/commands/build_constants.rs");
        let constants_path = project_root.join("crates/zos-bootstrap/src/constants.rs");

        let mut extracted_names = Vec::new();
        Self::extract_constant_names(&build_constants_path, &mut extracted_names)?;
        Self::extract_constant_names(&constants_path, &mut extracted_names)?;

        for name in &extracted_names {
            constant_counts.insert(name.clone(), 0);
        }

        Ok(ConstantAnalyzer { constant_counts })
    }

    fn extract_constant_names(file_path: &Path, constants: &mut Vec<String>) -> Result<()> {
        let content = fs::read_to_string(file_path)?;
        let syntax = syn::parse_file(&content)?;

        for item in syntax.items {
            if let Item::Const(item_const) = item {
                constants.push(item_const.ident.to_string());
            }
        }
        Ok(())
    }

    pub fn analyze(&mut self, search_dir: &Path) -> Result<()> {
        for entry in walkdir::WalkDir::new(search_dir) {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
                let content = fs::read_to_string(path)?;
                let syntax = syn::parse_file(&content)?;
                self.find_constant_usages(&syntax);
            }
        }
        Ok(())
    }

    fn find_constant_usages(&mut self, file: &syn::File) {
        for item in &file.items {
            match item {
                Item::Fn(item_fn) => {
                    for stmt in &item_fn.block.stmts {
                        self.find_paths_in_stmt(stmt);
                    }
                }
                Item::Use(_) => {
                    // Ignore use statements for now
                }
                Item::Const(item_const) => {
                    self.find_paths_in_expr(&item_const.expr);
                }
                Item::Static(item_static) => {
                    self.find_paths_in_expr(&item_static.expr);
                }
                _ => {
                    // Handle other item types as needed
                }
            }
        }
    }

    fn find_paths_in_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Local(local) => {
                if let Some(init) = &local.init {
                    self.find_paths_in_expr(&init.expr);
                }
            }
            Stmt::Expr(expr, _) => { // Match both fields of Stmt::Expr
                self.find_paths_in_expr(expr);
            }
            _ => {
                // Handle other statement types as needed
            }
        }
    }

    fn find_paths_in_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Path(expr_path) => {
                self.check_path(&expr_path.path);
            }
            Expr::Call(expr_call) => {
                self.find_paths_in_expr(&expr_call.func);
                for arg in &expr_call.args {
                    self.find_paths_in_expr(arg);
                }
            }
            Expr::MethodCall(expr_method_call) => {
                self.find_paths_in_expr(&expr_method_call.receiver);
                for arg in &expr_method_call.args {
                    self.find_paths_in_expr(arg);
                }
            }
            Expr::Block(expr_block) => {
                for stmt in &expr_block.block.stmts {
                    self.find_paths_in_stmt(stmt);
                }
            }
            Expr::Assign(expr_assign) => {
                self.find_paths_in_expr(&expr_assign.left);
                self.find_paths_in_expr(&expr_assign.right);
            }
            Expr::Binary(expr_binary) => {
                self.find_paths_in_expr(&expr_binary.left);
                self.find_paths_in_expr(&expr_binary.right);
            }
            Expr::Unary(expr_unary) => {
                self.find_paths_in_expr(&expr_unary.expr);
            }
            Expr::Lit(_) => {
                // Literal, no paths here
            }
            _ => {
                // Handle other expression types as needed
            }
        }
    }

    fn check_path(&mut self, path: &syn::Path) {
        if path.segments.len() == 2 {
            let module_name = path.segments[0].ident.to_string();
            if module_name == "build_constants" || module_name == "constants" {
                let constant_ident = &path.segments[1].ident;
                let constant_name = constant_ident.to_string();
                if self.constant_counts.contains_key(&constant_name) {
                    *self.constant_counts.get_mut(&constant_name).unwrap() += 1;
                }
            }
        }
    }

    pub fn report(&self) {
        println!("--- Constant Usage Report ---");
        for (constant_name, count) in &self.constant_counts {
            if *count > 1 {
                println!("FLAGGED: Constant '{}' used {} times.", constant_name, count);
            } else {
                println!("Constant '{}' used {} time(s).", constant_name, count);
            }
        }
        println!("-----------------------------");
    }
}