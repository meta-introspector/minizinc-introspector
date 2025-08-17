use std::{
    collections::HashMap,
    path::{PathBuf},
};
use syn::{
    visit::{self, Visit},
    Ident,
    ItemConst,
    ItemStatic,
};
use walkdir::WalkDir;


mod constants;
mod messages;
use ::message;


/// Struct to hold information about a constant declaration and its usage.
#[derive(Debug, Clone)]
struct ConstantInfo {
    name: String,
    declaration_path: PathBuf,
    usage_count: usize,
}

/// A `syn` visitor to find constant declarations and collect all identifiers.
struct ConstantVisitor {
    /// Stores declared constants found in the current file being visited.
    declared_constants_in_file: HashMap<String, PathBuf>,
    /// Collects all identifiers encountered in the current file.
    all_identifiers_in_file: Vec<String>,
    /// The path of the file currently being processed by this visitor.
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
        self.declared_constants_in_file
            .insert(name, self.current_file_path.clone());
        // Continue visiting children nodes of the const declaration
        visit::visit_item_const(self, i);
    }

    /// Visit `static` item declarations.
    fn visit_item_static(&mut self, i: &'ast ItemStatic) {
        let name = i.ident.to_string();
        self.declared_constants_in_file
            .insert(name, self.current_file_path.clone());
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

    // --- Phase 1: Code Ingestion and AST Parsing ---
    // Stores all unique constant declarations found across the entire project.
    // Key: Constant name (String), Value: ConstantInfo struct.
    let mut all_constant_declarations: HashMap<String, ConstantInfo> = HashMap::new();
    // Stores all identifiers encountered across the entire project.
    let mut all_identifiers_across_project: Vec<String> = Vec::new();

    message!(Phase1, &project_root);

    // Walk through the directory to find all Rust files.
    for entry in WalkDir::new(&project_root)
        .into_iter()
        .filter_map(|e| e.ok()) // Filter out errors
    {
        let path = entry.path();

        // Process only Rust files.
        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            message!(ProcessingFile, &path);
            let content = std::fs::read_to_string(path)?;
            let ast = syn::parse_file(&content)?;

            let mut visitor = ConstantVisitor::new(path.to_path_buf());
            visitor.visit_file(&ast); // Traverse the AST with our visitor.

            // Merge constant declarations found in this file into the global map.
            for (name, decl_path) in visitor.declared_constants_in_file {
                all_constant_declarations
                    .entry(name.clone())
                    .or_insert_with(|| ConstantInfo {
                        name,
                        declaration_path: decl_path,
                        usage_count: 0, // Usage count will be updated in Phase 2
                    });
            }
            // Collect all identifiers from this file into the global list.
            all_identifiers_across_project.extend(visitor.all_identifiers_in_file);
        }
    }

    // --- Phase 2: Constant Usage Analysis ---
    message!(Phase2);

    // Iterate through all declared constants and count their occurrences in the global list of identifiers.
    for (const_name, const_info) in all_constant_declarations.iter_mut() {
        let count = all_identifiers_across_project
            .iter()
            .filter(|id| *id == const_name) // Filter for identifiers that match the constant's name
            .count();
        const_info.usage_count = count;
    }

    // --- Phase 3: Constant Location and Refactoring Planning (Output to file) ---
    message!(Phase3);
    message!(WritingReport, &output_file_path);

    let mut report_content = String::new();
    report_content.push_str(message!(ReportHeader));

    // Sort constants by name for consistent reporting
    let mut sorted_constants: Vec<(&String, &ConstantInfo)> = 
        all_constant_declarations.iter().collect();
    sorted_constants.sort_by_key(|(name, _)| *name);

    for (name, info) in sorted_constants {
        report_content.push_str(&message!(ConstantDetail, name, &info.declaration_path, info.usage_count));

        // Flag constants used more than once.
        if info.usage_count > 1 {
            report_content.push_str(message!(StatusFlagged));
        } else {
            report_content.push_str(message!(StatusOK));
        }

        // Determine if the constant is in a dedicated constants file.
        let file_name = info
            .declaration_path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        let parent_dir_name = info
            .declaration_path
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

        if info.usage_count > 1 && !is_dedicated_constants_file {
            report_content.push_str(message!(PlanMoveConstant));
        } else if info.usage_count <= 1 && !is_dedicated_constants_file {
            report_content.push_str(message!(NoteSingleUseNotDedicated));
        } else if is_dedicated_constants_file {
            report_content.push_str(message!(NoteAlreadyDedicated));
        }
        report_content.push_str(message!(ReportSeparator));
    }

    // Write the generated report content to the specified output file.
    std::fs::write(&output_file_path, report_content)?;
    message!(AnalysisComplete, &output_file_path);

    Ok(())
}
