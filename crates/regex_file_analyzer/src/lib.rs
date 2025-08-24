use std::collections::HashSet;
use std::path::Path;
use serde::{Deserialize, Serialize};
//use anyhow::{Result, anyhow};

// --- Rust representation of MiniZinc Enums and Records ---

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub enum ComponentType {
    Crate,
    Doc,
    Vendor,
    Tool,
    Other,
}

impl std::fmt::Display for ComponentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_uppercase().trim_matches('"'))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub enum FileType {
    RustSource,
    Markdown,
    Json,
    Toml,
    ShellScript,
    MznModel,
    DznData,
    OtherFile,
}

impl std::fmt::Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_uppercase().trim_matches('"'))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectFile {
    pub path: String,
    pub file_type: FileType,
    pub associated_regex_idx: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectComponent {
    pub name: String,
    pub component_type: ComponentType,
    pub files: HashSet<usize>, // Indices into the global all_files vector
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectModelData {
    pub all_files: Vec<ProjectFile>,
    pub all_regex_patterns: Vec<String>,
    pub components: Vec<ProjectComponent>,
}

// --- Helper functions for categorization ---

pub fn get_file_type(path: &Path) -> FileType {
    match path.extension().and_then(|s| s.to_str()) {
        Some("rs") => FileType::RustSource,
        Some("md") => FileType::Markdown,
        Some("json") => FileType::Json,
        Some("toml") => FileType::Toml,
        Some("sh") | Some("bash") => FileType::ShellScript,
        Some("mzn") => FileType::MznModel,
        Some("dzn") => FileType::DznData,
        _ => FileType::OtherFile,
    }
}

pub fn get_component_type(path: &Path) -> ComponentType {
    let path_str = path.to_str().unwrap_or("");
    if path_str.contains("/crates/") {
        ComponentType::Crate
    } else if path_str.contains("/docs/") {
        ComponentType::Doc
    } else if path_str.contains("/vendor/") {
        ComponentType::Vendor
    } else if path_str.contains("/tools/") {
        ComponentType::Tool
    } else {
        ComponentType::Other
    }
}

// --- Struct for deserializing generated_path_regexes.json ---
#[derive(Serialize, Deserialize, Debug)]
pub struct LineRegex {
    pub line: String,
    pub regex: String,
}

// Helper function to convert JSON to MiniZinc DZN format
pub fn json_to_dzn(json_str: &str) -> String {
    let project_model_data: ProjectModelData = serde_json::from_str(json_str)
        .expect("Failed to deserialize JSON to ProjectModelData");

    let mut dzn_content = String::new();

    // all_files
    dzn_content.push_str("all_files = [
");
    for (i, file) in project_model_data.all_files.iter().enumerate() {
        let escaped_path = file.path.replace("\\", "\\\\");
        dzn_content.push_str(&format!("  (path: \"{}\", file_type: {}, associated_regex_idx: {})",
            escaped_path,
            file.file_type, // Now handled by Display impl
            file.associated_regex_idx,
        ));
        if i < project_model_data.all_files.len() - 1 {
            dzn_content.push_str(",\n");
        } else {
            dzn_content.push_str("\n");
        }
    }
    dzn_content.push_str("];\n\n");

    // all_regex_patterns
    dzn_content.push_str("all_regex_patterns = [
");
    for (i, pattern) in project_model_data.all_regex_patterns.iter().enumerate() {
        let escaped_pattern = pattern.replace("\\", "\\\\");
        dzn_content.push_str(&format!("  \"{}\"",
            escaped_pattern,
        ));
        if i < project_model_data.all_regex_patterns.len() - 1 {
            dzn_content.push_str(",\n");
        } else {
            dzn_content.push_str("\n");
        }
    }
    dzn_content.push_str("];\n\n");

    // components
    dzn_content.push_str("components = [
");
    for (i, component) in project_model_data.components.iter().enumerate() {
        let escaped_name = component.name.replace("\\", "\\\\");
        let files_str = component.files.iter()
            .map(|&idx| idx.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        dzn_content.push_str(&format!("  (name: \"{}\", component_type: {}, files: {{{}}})",
            escaped_name,
            component.component_type, // Now handled by Display impl
            files_str,
        ));
        if i < project_model_data.components.len() - 1 {
            dzn_content.push_str(",\n");
        } else {
            dzn_content.push_str("\n");
        }
    }
    dzn_content.push_str("];\n\n");

    dzn_content
}
