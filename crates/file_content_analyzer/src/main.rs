use walkdir::WalkDir;
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use regex::Regex;
use toml::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct FileAnalysis {
    path: PathBuf,
    word_count: usize,
    bag_of_words: HashMap<String, usize>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ProjectAnalysis {
    project_root: PathBuf,
    rust_files: Vec<FileAnalysis>,
}

fn tokenize(text: &str) -> Vec<String> {
    let re = Regex::new(r"\b\w+\b").unwrap();
    re.find_iter(text)
        .map(|m| m.as_str().to_lowercase())
        .collect()
}

fn calculate_cosine_similarity(map1: &HashMap<String, usize>, map2: &HashMap<String, usize>) -> f64 {
    let mut dot_product = 0.0;
    let mut magnitude1 = 0.0;
    let mut magnitude2 = 0.0;

    let all_words: std::collections::HashSet<String> = map1.keys().chain(map2.keys()).cloned().collect();

    for word in all_words {
        let count1 = *map1.get(&word).unwrap_or(&0) as f64;
        let count2 = *map2.get(&word).unwrap_or(&0) as f64;

        dot_product += count1 * count2;
        magnitude1 += count1.powi(2);
        magnitude2 += count2.powi(2);
    }

    let magnitude1 = magnitude1.sqrt();
    let magnitude2 = magnitude2.sqrt();

    if magnitude1 == 0.0 || magnitude2 == 0.0 {
        return 0.0;
    }

    dot_product / (magnitude1 * magnitude2)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let search_root = PathBuf::from("/data/data/com.termux/files/home/storage/github/");
    let cache_file = search_root.join("file_analysis_cache.json");

    let mut all_project_analyses: Vec<ProjectAnalysis> = Vec::new();
    let mut all_rust_files: Vec<FileAnalysis> = Vec::new();
    let mut processed_project_roots: std::collections::HashSet<PathBuf> = std::collections::HashSet::new();

    // Attempt to load existing state from cache
    if cache_file.exists() {
        eprintln!("Loading existing analysis from cache: {:?}", cache_file);
        let cached_data = fs::read_to_string(&cache_file)?;
        all_project_analyses = serde_json::from_str(&cached_data)?;

        for project_analysis in &all_project_analyses {
            processed_project_roots.insert(project_analysis.project_root.clone());
            for file_analysis in &project_analysis.rust_files {
                all_rust_files.push(file_analysis.clone());
            }
        }
        eprintln!("Loaded {} projects and {} Rust files from cache.", all_project_analyses.len(), all_rust_files.len());
    }

    eprintln!("Discovering Rust projects in: {:?}", search_root);

    // First pass: Discover Cargo.toml files to identify project roots
    let mut discovered_project_roots: std::collections::HashSet<PathBuf> = std::collections::HashSet::new();
    for entry in WalkDir::new(&search_root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.path().components().any(|comp| comp.as_os_str() == "target")) // Skip target directories
    {
        let path = entry.path();
        if path.is_file() && path.file_name().map_or(false, |name| name == "Cargo.toml") {
            if let Some(parent) = path.parent() {
                discovered_project_roots.insert(parent.to_path_buf());
            }
        }
    }

    eprintln!("Found {} potential Rust project roots.", discovered_project_roots.len());

    // Second pass: Process .rs files within identified project roots, skipping already processed ones
    for project_root in discovered_project_roots {
        if processed_project_roots.contains(&project_root) {
            eprintln!("Skipping already processed project: {:?}", project_root);
            continue;
        }

        eprintln!("Processing new Rust project: {:?}", project_root);
        let mut current_project_rust_files: Vec<FileAnalysis> = Vec::new();

        let cargo_toml_path = project_root.join("Cargo.toml");
        let mut prioritized_files: std::collections::HashSet<PathBuf> = std::collections::HashSet::new();

        // Parse Cargo.toml to find prioritized files
        if let Ok(cargo_toml_content) = fs::read_to_string(&cargo_toml_path) {
            if let Ok(parsed_toml) = cargo_toml_content.parse::<Value>() {
                // Add lib.rs if it exists
                if let Some(lib_path) = parsed_toml.get("lib").and_then(|lib| lib.get("path")).and_then(|path| path.as_str()) {
                    prioritized_files.insert(project_root.join(lib_path));
                } else {
                    prioritized_files.insert(project_root.join("src/lib.rs"));
                }

                // Add main.rs if it exists
                if let Some(bin_array) = parsed_toml.get("bin").and_then(|bin| bin.as_array()) {
                    for bin_entry in bin_array {
                        if let Some(path) = bin_entry.get("path").and_then(|path| path.as_str()) {
                            prioritized_files.insert(project_root.join(path));
                        }
                    }
                } else {
                    prioritized_files.insert(project_root.join("src/main.rs"));
                }
            }
        }

        // Process prioritized files first
        for path in &prioritized_files {
            if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
                match fs::read_to_string(path) {
                    Ok(file_content) => {
                        let tokens = tokenize(&file_content);
                        let mut bag_of_words: HashMap<String, usize> = HashMap::new();
                        for token in &tokens {
                            *bag_of_words.entry(token.clone()).or_insert(0) += 1;
                        }
                        let word_count = tokens.len();

                        let file_analysis = FileAnalysis {
                            path: path.to_path_buf(),
                            word_count,
                            bag_of_words,
                        };
                        current_project_rust_files.push(file_analysis.clone());
                        all_rust_files.push(file_analysis);
                    },
                    Err(e) => {
                        eprintln!("  Error reading prioritized Rust file {:?}: {}", path, e);
                    }
                }
            }
        }

        // Process remaining .rs files
        for entry in WalkDir::new(&project_root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| !e.path().components().any(|comp| comp.as_os_str() == "target")) // Skip target directories
        {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") && !prioritized_files.contains(path) {
                match fs::read_to_string(path) {
                    Ok(file_content) => {
                        let tokens = tokenize(&file_content);
                        let mut bag_of_words: HashMap<String, usize> = HashMap::new();
                        for token in &tokens {
                            *bag_of_words.entry(token.clone()).or_insert(0) += 1;
                        }
                        let word_count = tokens.len();

                        let file_analysis = FileAnalysis {
                            path: path.to_path_buf(),
                            word_count,
                            bag_of_words,
                        };
                        current_project_rust_files.push(file_analysis.clone());
                        all_rust_files.push(file_analysis);
                    },
                    Err(e) => {
                        eprintln!("  Error reading Rust file {:?}: {}", path, e);
                    }
                }
            }
        }
        all_project_analyses.push(ProjectAnalysis {
            project_root: project_root.clone(),
            rust_files: current_project_rust_files,
        });

        // Save state incrementally after each project
        eprintln!("Saving incremental cache to {:?}", cache_file);
        let serialized = serde_json::to_string_pretty(&all_project_analyses)?;
        fs::write(&cache_file, serialized)?;
    }

    eprintln!("All project analysis complete. Finalizing similarity calculation...");

    eprintln!("Calculating similarities among all .rs files...");
    for i in 0..all_rust_files.len() {
        for j in (i + 1)..all_rust_files.len() {
            let file1 = &all_rust_files[i];
            let file2 = &all_rust_files[j];

            let similarity = calculate_cosine_similarity(&file1.bag_of_words, &file2.bag_of_words);

            if similarity >= 0.90 {
                println!("\n--- 90% Similar Rust Files ---");
                println!("File 1: {:?}", file1.path);
                println!("File 2: {:?}", file2.path);
                println!("Similarity: {:.2}%", similarity * 100.0);
            }
        }
    }

    eprintln!("Similarity calculation complete.");

    Ok(())
}
