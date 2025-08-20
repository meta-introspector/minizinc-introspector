use std::collections::HashSet;
use std::fs;
use std::path::{
    //Path,
    PathBuf};
use syn::{self, visit::{self, Visit}};

use crate::utils::error::Result;
//use crate::utils::paths;

pub struct ConstantUsageVisitor {
    pub used_constants: HashSet<String>,
}

impl<'ast> Visit<'ast> for ConstantUsageVisitor {
    fn visit_path(&mut self, path: &'ast syn::Path) {
        if let Some(segment) = path.segments.first() {
            if segment.ident == "constants" {
                if let Some(constant_name_segment) = path.segments.get(1) {
                    self.used_constants.insert(constant_name_segment.ident.to_string());
                }
            }
        }
        visit::visit_path(self, path);
    }
}

pub fn prove_constants_usage_command(project_root: &PathBuf) -> Result<()> {
    println!("Proving constant usage in constants.rs...");

    let mut defined_constants: HashSet<String> = HashSet::new();
    let constants_file_path = project_root.join("crates/zos-bootstrap/src/constants.rs");
    let constants_file_content = fs::read_to_string(&constants_file_path)?;
    let constants_syntax = syn::parse_file(&constants_file_content)?;

    // Extract defined constants from constants.rs
    for item in constants_syntax.items {
        if let syn::Item::Const(item_const) = item {
            defined_constants.insert(item_const.ident.to_string());
        }
    }

    let mut used_constants: HashSet<String> = HashSet::new();

    // Glob for all .rs files in the project
    let rust_files = glob::glob(&format!("{}/crates/**/*.rs", project_root.display()))?
        .filter_map(std::result::Result::ok);

    for entry in rust_files {
        let file_path = entry.as_path();
        let file_content = fs::read_to_string(file_path)?;
        let file_syntax = syn::parse_file(&file_content)?;

        let mut visitor = ConstantUsageVisitor { used_constants: HashSet::new() };
        visitor.visit_file(&file_syntax);
        used_constants.extend(visitor.used_constants);
    }

    let mut unused_constants: Vec<String> = Vec::new();
    for defined_constant in &defined_constants {
        if !used_constants.contains(defined_constant) {
            unused_constants.push(defined_constant.clone());
        }
    }

    if unused_constants.is_empty() {
        println!("All constants in constants.rs are proven to be used.");
    } else {
        println!("The following constants in constants.rs are NOT used:");
        for unused_constant in unused_constants {
            println!("- {}", unused_constant);
        }
    }

    Ok(())
}
