use std::{
    collections::HashMap,
    path::{PathBuf},
    hash::{Hash, Hasher},
    collections::hash_map::DefaultHasher,
};
use syn::{
    visit::{self, Visit},
    Ident,
    ItemConst,
    ItemStatic,
};
use walkdir::WalkDir;
//use minizinc_ffi::MiniZincEnvironment;


mod constants;
#[macro_use]
mod messages;


/// Struct to hold information about a constant declaration and its usage.
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct ConstantInfo {
    name: String,
    declaration_path: PathBuf,
    usage_count: usize,
    value: String,
}

fn extract_literal_value(lit: &syn::Lit) -> String {
    match lit {
        syn::Lit::Str(lit_str) => format!("{}", lit_str.value()),
        syn::Lit::ByteStr(lit_byte_str) => format!("{}", String::from_utf8_lossy(&lit_byte_str.value())),
        syn::Lit::Byte(lit_byte) => format!("{}", lit_byte.value()),
        syn::Lit::Char(lit_char) => format!("'{}'", lit_char.value()),
        syn::Lit::Int(lit_int) => lit_int.to_string(),
        syn::Lit::Float(lit_float) => lit_float.to_string(),
        syn::Lit::Bool(lit_bool) => lit_bool.value.to_string(),
        _ => String::from("[Unsupported literal type]"),
    }
}

fn clean_name(s: &str) -> String {
    let mut cleaned = String::new();
    for c in s.chars() {
        if c.is_alphanumeric() {
            cleaned.push(c.to_ascii_lowercase());
        } else {
            cleaned.push('_');
        }
    }
    // Remove leading/trailing underscores and replace multiple underscores with a single one
    let cleaned = cleaned.trim_matches('_').to_string();
    let cleaned = cleaned.replace("__", "_");

    // Ensure it starts with a letter or underscore
    if cleaned.is_empty() || cleaned.chars().next().unwrap().is_numeric() {
        format!("_{}", cleaned)
    } else {
        cleaned
    }
}

// fn clean_name2(s: &str) -> String {
//     let mut cleaned = String::new();
//     for c in s.chars() {
//         if c.is_alphanumeric() {
//             cleaned.push(c.to_ascii_lowercase());
//         } else {
//             cleaned.push('_');
//         }
//     }
//     // Remove leading/trailing underscores and replace multiple underscores with a single one
//     let cleaned = cleaned.trim_matches('_').to_string();
//     let cleaned = cleaned.replace("__", "_");

//     // Ensure it starts with a letter or underscore
//     if cleaned.is_empty() || cleaned.chars().next().unwrap().is_numeric() {
//         format!("_{}", cleaned)
//     } else {
//         cleaned
//     }
// }

/// A `syn` visitor to find constant declarations and collect all identifiers.
struct ConstantVisitor {
    declared_constants_in_file: HashMap<String, (PathBuf, String)>,
    all_identifiers_in_file: Vec<String>,
    current_file_path: PathBuf,
}

impl ConstantVisitor {
    fn new(file_path: PathBuf) -> Self {
        ConstantVisitor {
            declared_constants_in_file: HashMap::new(),
            all_identifiers_in_file: Vec::new(),
            current_file_path: file_path,
        }
    }
}

/// Implement the `Visit` trait to traverse the AST and extract information.
impl<'ast> Visit<'ast> for ConstantVisitor {
    /// Visit `const` item declarations.
    fn visit_item_const(&mut self, i: &'ast ItemConst) {
        let name = i.ident.to_string();
        let value = if let syn::Expr::Lit(expr_lit) = &*i.expr {
            extract_literal_value(&expr_lit.lit)
        } else {
            String::from("[Non-literal constant value]")
        };
        self.declared_constants_in_file
            .insert(name, (self.current_file_path.clone(), value));
        // Continue visiting children nodes of the const declaration
        visit::visit_item_const(self, i);
    }

    /// Visit `static` item declarations.
    fn visit_item_static(&mut self, i: &'ast ItemStatic) {
        let name = i.ident.to_string();
        let value = if let syn::Expr::Lit(expr_lit) = &*i.expr {
            extract_literal_value(&expr_lit.lit)
        } else {
            String::from("[Non-literal static value]")
        };
        self.declared_constants_in_file
            .insert(name, (self.current_file_path.clone(), value));
        // Continue visiting children nodes of the static declaration
        visit::visit_item_static(self, i);
    }

    /// Visit any identifier. This is how we track usage.
    fn visit_ident(&mut self, i: &'ast Ident) {
        self.all_identifiers_in_file.push(i.to_string());
        // Continue visiting children nodes of the identifier
        visit::visit_ident(self, i);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the project root directory. This should be the root of your Rust project.
    // For this specific context, it's /data/data/com.termux/files/home/storage/github/libminizinc
    let project_root = constants::get_project_root()?;

    // --- Command-line argument for output file ---
    let args: Vec<String> = std::env::args().collect();
    let output_file_path = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        // Default output path: reports/constant_analysis_report.txt
        let mut default_path = PathBuf::from("./reports");
        // Ensure the 'reports' directory exists
        std::fs::create_dir_all(&default_path)?;
        default_path.push("constant_analysis_report.txt");
        default_path
    };

    let mut output_constants_dir: Option<PathBuf> = None;
    // Basic argument parsing for --output-constants-dir
    let mut args_iter = args.iter().skip(1).peekable(); // Skip program name
    while let Some(arg) = args_iter.next() {
        if arg == "--output-constants-dir" {
            if let Some(dir_path) = args_iter.next() {
                output_constants_dir = Some(PathBuf::from(dir_path));
            } else {
                return Err("Missing path for --output-constants-dir".into());
            }
        }
    }

    if let Some(ref path) = output_constants_dir {
        std::fs::create_dir_all(path)?;
    }

    // let mut naming_request_file_path: Option<PathBuf> = None;
    // // Basic argument parsing for --naming-request-file
    // let mut args_iter = args.iter().skip(1).peekable(); // Skip program name
    // while let Some(arg) = args_iter.next() {
    //     if arg == "--naming-request-file" {
    //         if let Some(file_path) = args_iter.next() {
    //             naming_request_file_path = Some(PathBuf::from(file_path));
    //             println!("DEBUG: naming_request_file_path set to: {:?}", naming_request_file_path);
    //         } else {
    //             return Err("Missing path for --naming-request-file".into());
    //         }
    //     }
    // }

    // --- Phase 1: Code Ingestion and AST Parsing ---
    // Stores all unique constant declarations found across the entire project.
    // Key: Constant name (String), Value: ConstantInfo struct.
    let mut all_constant_declarations: HashMap<String, ConstantInfo> = HashMap::new();
    // Stores all identifiers encountered across the entire project.
    let mut all_identifiers_across_project: Vec<String> = Vec::new();

    crate::message!(Phase1, &project_root);

    // Walk through the directory to find all Rust files.
    for entry in WalkDir::new(&project_root)
        .into_iter()
        .filter_map(|e| e.ok()) // Filter out errors
    {
        let path = entry.path();

        // Process only Rust files.
        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            crate::message!(ProcessingFile, &path);
            let content = std::fs::read_to_string(path)?;
            let ast = syn::parse_file(&content)?;

            let mut visitor = ConstantVisitor::new(path.to_path_buf());
            visitor.visit_file(&ast); // Traverse the AST with our visitor.

            // Merge constant declarations found in this file into the global map.
            for (name, (decl_path, value)) in visitor.declared_constants_in_file {
                all_constant_declarations
                    .entry(name.clone())
                    .or_insert_with(|| ConstantInfo {
                        name,
                        declaration_path: decl_path,
                        usage_count: 0, // Usage count will be updated in Phase 2
                        value,
                    });
            }
            // Collect all identifiers from this file into the global list.
            all_identifiers_across_project.extend(visitor.all_identifiers_in_file);
        }
    }

    // --- Phase 2: Constant Usage Analysis ---
    crate::message!(Phase2);

    // Stores unique literal values and their declaration info.
    // Key: Literal value (String), Value: Vector of (constant name, declaration path)
    let mut unique_literal_values: HashMap<String, Vec<(String, PathBuf)>> = HashMap::new();

    for (const_name, const_info) in all_constant_declarations.into_iter() { // Use into_iter to consume
        unique_literal_values
            .entry(const_info.value.clone())
            .or_insert_with(Vec::new)
            .push((const_name, const_info.declaration_path));
    }

    // --- Phase 3: Constant Location and Refactoring Planning (Output to file) ---
    crate::message!(Phase3);
    crate::message!(WritingReport, &output_file_path);

    let mut report_content = String::new();
    report_content.push_str(crate::message!(ReportHeader));

    // Sort unique literal values for consistent reporting
    let mut sorted_literal_values: Vec<(&String, &Vec<(String, PathBuf)>)> = 
        unique_literal_values.iter().collect();
    sorted_literal_values.sort_by_key(|(value, _)| *value);

    for (value, declarations) in sorted_literal_values {
        // Report for each unique literal value
        report_content.push_str(&crate::message!(ConstantDetail, value, &declarations[0].1, declarations.len())); // Use value as name, first decl path, and count of declarations

        // Flag if the literal value is declared more than once
        if declarations.len() > 1 {
            report_content.push_str(crate::message!(StatusFlagged));
            report_content.push_str(&format!("  PLAN: This literal value is declared {} times. Consider centralizing its definition.\n", declarations.len()));
        } else {
            report_content.push_str(crate::message!(StatusOK));
        }

        // Generate individual constant files if output_constants_dir is provided
        if let Some(ref output_dir) = output_constants_dir {
            let mut hasher = DefaultHasher::new();
            value.hash(&mut hasher);
            let file_name_hash = format!("constant_{:x}", hasher.finish());

            let cleaned_name = clean_name(value);
            let const_name = if cleaned_name.is_empty() {
                file_name_hash.to_uppercase() // Fallback to hash if cleaned name is empty
            } else {
                cleaned_name.to_uppercase()
            };

            let file_name = format!("{}.rs", cleaned_name);
            let file_path = output_dir.join(&file_name);

            let const_content = format!("pub const {}: &str = {};", const_name, value);

            std::fs::write(&file_path, const_content)?;
        }

        // Determine if the constant is in a dedicated constants file.
        let file_name = declarations[0].1
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        let parent_dir_name = declarations[0].1
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|s| s.to_str())
            .unwrap_or("");

        let is_dedicated_constants_file = file_name.contains("constants.rs")
            || file_name.contains("config.rs")
            || file_name.contains("types.rs")
            || parent_dir_name.contains("constants")
            || parent_dir_name.contains("config")
            || parent_dir_name.contains("types");

        if declarations.len() > 1 && !is_dedicated_constants_file {
            report_content.push_str(crate::message!(PlanMoveConstant));
        } else if declarations.len() <= 1 && !is_dedicated_constants_file {
            report_content.push_str(crate::message!(NoteSingleUseNotDedicated));
        } else if is_dedicated_constants_file {
            report_content.push_str(crate::message!(NoteAlreadyDedicated));
        }
        report_content.push_str(crate::message!(ReportSeparator));
    }

    // Write the generated report content to the specified output file.
    std::fs::write(&output_file_path, report_content)?;
    crate::message!(AnalysisComplete, &output_file_path);

    Ok(())
}
