use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;
use anyhow::{Result, anyhow};

use regex_file_analyzer::{
    LineRegex,
    ProjectFile,
    ProjectComponent,
    ProjectModelData,
    get_file_type,
    get_component_type,
    json_to_dzn,
};

fn main() -> Result<()> {
    let generated_regexes_path = "/data/data/com.termux/files/home/storage/github/libminizinc/generated_path_regexes.json";
    let file_analysis_summary_path = "/data/data/com.termux/files/home/storage/github/libminizinc/.file_analysis_summary_index.txt";
    let output_dzn_path = "/data/data/com.termux/files/home/storage/github/libminizinc/project_model_data.dzn";

    // 1. Read and parse the generated regexes JSON file
    let regexes_json_content = fs::read_to_string(generated_regexes_path)
        .map_err(|e| anyhow!("Failed to read generated regexes file {:?}: {}", generated_regexes_path, e))?;
    let line_regexes: Vec<LineRegex> = serde_json::from_str(&regexes_json_content)
        .map_err(|e| anyhow!("Failed to parse generated regexes JSON: {}", e))?;

    // Store all unique regex patterns and map them to indices
    let mut all_regex_patterns: Vec<String> = Vec::new();
    let mut regex_to_idx: HashMap<String, usize> = HashMap::new();
    for lr in &line_regexes {
        if !regex_to_idx.contains_key(&lr.regex) {
            regex_to_idx.insert(lr.regex.clone(), all_regex_patterns.len());
            all_regex_patterns.push(lr.regex.clone());
        }
    }

    // 2. Create ProjectFile instances from line_regexes
    let mut all_files: Vec<ProjectFile> = Vec::new();
    let mut path_to_file_idx: HashMap<String, usize> = HashMap::new();

    for (i, lr) in line_regexes.iter().enumerate() {
        let path_str = &lr.line;
        let path = PathBuf::from(path_str);
        let file_type = get_file_type(&path);
        let associated_regex_idx = *regex_to_idx.get(&lr.regex)
            .expect("Regex should already be in regex_to_idx map");

        let project_file = ProjectFile {
            path: path_str.clone(),
            file_type,
            associated_regex_idx,
        };
        all_files.push(project_file);
        path_to_file_idx.insert(path_str.clone(), i);
    }

    // 4. Group files into ProjectComponent instances
    let mut components_map: HashMap<String, ProjectComponent> = HashMap::new();

    for (file_idx, project_file) in all_files.iter().enumerate() {
        let component_name_prefix = if project_file.path.contains("/crates/") {
            let parts: Vec<&str> = project_file.path.split("/crates/").collect();
            if parts.len() > 1 {
                let crate_path_part = parts[1];
                let crate_name = crate_path_part.split('/').next().unwrap_or("").to_string();
                format!("crates/{}", crate_name)
            } else {
                "other_crates".to_string()
            }
        } else if project_file.path.contains("/docs/") {
            "docs".to_string()
        } else if project_file.path.contains("/vendor/") {
            "vendor".to_string()
        } else if project_file.path.contains("/tools/") {
            "tools".to_string()
        } else {
            "root_level".to_string()
        };

        let component_type = get_component_type(&PathBuf::from(&project_file.path));

        let component = components_map.entry(component_name_prefix.clone()).or_insert_with(|| ProjectComponent {
            name: component_name_prefix,
            component_type,
            files: HashSet::new(),
        });
        component.files.insert(file_idx);
    }

    let mut components: Vec<ProjectComponent> = components_map.into_values().collect();
    // Sort components for consistent output
    components.sort_by(|a, b| a.name.cmp(&b.name));

    // Create the top-level ProjectModelData struct
    let project_model_data = ProjectModelData {
        all_files,
        all_regex_patterns,
        components,
    };

    // Serialize to JSON
    let json_content = serde_json::to_string_pretty(&project_model_data)
        .map_err(|e| anyhow!("Failed to serialize project model to JSON: {}", e))?;

    // Convert JSON to MiniZinc DZN format
    let dzn_content = json_to_dzn(&json_content);

    // 6. Write the .dzn content to file
    fs::write(output_dzn_path, dzn_content)
        .map_err(|e| anyhow!("Failed to write DZN output to {:?}: {}", output_dzn_path, e))?;

    println!("Successfully generated MiniZinc data file: {}", output_dzn_path);

    Ok(())
}