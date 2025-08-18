use std::{fs, path::{Path, PathBuf}};
use walkdir::WalkDir;
use std::collections::HashMap;

// Define a struct to hold our category mappings
struct DocCategory {
    prime: u32,
    theme_name: String, // Human-readable theme name for directory
    keywords: Vec<String>, // Keywords to match against filename/content
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let docs_dir = PathBuf::from("/data/data/com.termux/files/home/storage/github/libminizinc/docs");
    let new_base_dir = PathBuf::from("/data/data/com.termux/files/home/storage/github/libminizinc/docs/categorized");

    // Define categories based on ZOS primes and project themes
    let categories = vec![
        DocCategory { prime: 2, theme_name: "Core_Code".to_string(), keywords: vec!["code".to_string(), "coding".to_string(), "rust".to_string(), "binary".to_string(), "computer".to_string(), "logic".to_string()] },
        DocCategory { prime: 3, theme_name: "Structure_Build".to_string(), keywords: vec!["build".to_string(), "architecture".to_string(), "structure".to_string(), "design".to_string(), "foundation".to_string()] },
        DocCategory { prime: 5, theme_name: "Organization_Process".to_string(), keywords: vec!["sop".to_string(), "process".to_string(), "methodology".to_string(), "organization".to_string(), "workflow".to_string(), "system".to_string()] },
        DocCategory { prime: 7, theme_name: "Completion_Goals".to_string(), keywords: vec!["goal".to_string(), "fixed_point".to_string(), "vision".to_string(), "completion".to_string(), "purpose".to_string(), "perfection".to_string()] },
        DocCategory { prime: 11, theme_name: "Modularity_Design".to_string(), keywords: vec!["module".to_string(), "design".to_string(), "pattern".to_string(), "modularity".to_string(), "component".to_string(), "parts".to_string()] },
        DocCategory { prime: 13, theme_name: "Integration_Bridging".to_string(), keywords: vec!["ffi".to_string(), "integration".to_string(), "bridge".to_string(), "connection".to_string(), "link".to_string(), "collaboration".to_string()] },
        DocCategory { prime: 17, theme_name: "Abstraction_Ontology".to_string(), keywords: vec!["ontology".to_string(), "abstract".to_string(), "schema".to_string(), "interface".to_string(), "concept".to_string(), "model".to_string()] },
        DocCategory { prime: 19, theme_name: "Iteration_Evolution".to_string(), keywords: vec!["iterative".to_string(), "evolution".to_string(), "cycle".to_string(), "refactoring".to_string(), "progress".to_string(), "transform".to_string()] },
        // Add more categories as needed, or a default one
    ];

    // Create the new base directory if it doesn't exist
    fs::create_dir_all(&new_base_dir)?;

    // Collect all files to process first to avoid issues with modifying directory during iteration
    let mut files_to_process = Vec::new();
    for entry in WalkDir::new(&docs_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
            // Exclude files already in the target categorized directory
            if !path.starts_with(&new_base_dir) {
                files_to_process.push(path.to_path_buf());
            }
        }
    }

    for path in files_to_process {
        let filename = path.file_name().unwrap().to_string_lossy().to_lowercase();
        let file_content = fs::read_to_string(&path).unwrap_or_default().to_lowercase();

        let mut assigned_category: Option<&DocCategory> = None;

        // Try to match by filename first
        for category in &categories {
            if category.keywords.iter().any(|k| filename.contains(k)) {
                assigned_category = Some(category);
                break;
            }
        }

        // If not matched by filename, try to match by content
        if assigned_category.is_none() {
            for category in &categories {
                if category.keywords.iter().any(|k| file_content.contains(k)) {
                    assigned_category = Some(category);
                    break;
                }
            }
        }

        // Default category if no match found
        let final_category = assigned_category.unwrap_or(&DocCategory {
            prime: 0,
            theme_name: "Uncategorized".to_string(),
            keywords: vec![], // No keywords for default
        });

        let new_subdir = new_base_dir
            .join(final_category.prime.to_string())
            .join(&final_category.theme_name);
        
        fs::create_dir_all(&new_subdir)?;
        let new_path = new_subdir.join(path.file_name().unwrap());

        // Only move if the file is not already in the correct new location
        if path != new_path {
            fs::rename(&path, &new_path)?;
            println!("Moved: {:?} to {:?}", path, new_path);
        } else {
            println!("Skipped: {:?} (already in correct location)", path);
        }
    }

    Ok(())
}