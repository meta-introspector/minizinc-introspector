//! # Rust File Finder and Code Analyzer
//! 
//! This tool provides a suite of utilities for analyzing Rust codebases.
//! It can be used to perform tasks such as:
//! 
//! * **Full Analysis:** Recursively finds all Rust projects in a directory,
//!   analyzes each file to create a bag-of-words representation,
//!   calculates cosine similarity between files.
//! * **Crate Similarity:** Compares a target crate to all other crates found
//!   in the analysis and reports the most similar ones.
//! * **Keyword Search:** Searches for keywords across the entire codebase
//!   and reports the files where they are found.
//! * **And more:** See the `Args` struct for a full list of modes.
//! 
//! The tool is designed to be modular and extensible, with different
//! functionalities encapsulated in separate `run_` functions.

use walkdir::WalkDir;
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use regex::Regex;
use toml::Value;
use clap::Parser;
use rayon::prelude::*;
use std::time::SystemTime;
use anyhow::Result;
use puffin;
//use puffin_egui;

const GITHUB_ROOT_DIR: &str = "/data/data/com.termux/files/home/storage/github/";

// Synonyms for SEO:
// Code Analysis
// Source Code Indexing
// Software Metrics

// SEO Experiment: Additional terms for discoverability (experimental, not standard practice)
// code search
// repository analysis
// source code intelligence
// file indexing
// text mining

/// Command-line arguments for the Rust File Finder and Code Analyzer.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Mode of operation: 'full_analysis', 'read_cargo_toml', 'crate_similarity', 'migrate_cache', 'search_keywords', 'generate_stopword_report', 'build_hierarchical_index', 'find_exact_shared_files_terms' (helps find common code across projects), or 'common_terms_report', or 'estimate'
    #[arg(short, long, default_value = "full_analysis")]
    mode: String,

    /// Target crate for similarity analysis (used with --mode crate_similarity)
    #[arg(long)]
    target_crate: Option<String>,

    /// Number of most similar crates to display (used with --mode crate_similarity or common_terms_report)
    #[arg(long, default_value_t = 10)]
    most_similar: usize,

    /// Keywords to search for (used with --mode search_keywords)
    #[arg(long, value_delimiter = ' ')]
    keywords: Vec<String>,

    /// Optional path to search within (used with --mode search_keywords, find_exact_shared_files_terms)
    #[arg(long)]
    search_path: Option<PathBuf>,

    /// Optional path to filter results by (used with --mode find_exact_shared_files_terms)
    #[arg(long)]
    filter_by_path: Option<PathBuf>,

    /// Crates to include in the common terms report (used with --mode common_terms_report)
    #[arg(long, value_delimiter = ' ')]
    crates: Vec<String>,

    /// Enable profiling and generate a profile report.
    #[arg(long)]
    profile: bool,
}

/// Represents the analysis of a single file.
#[derive(Debug, Serialize, Deserialize, Clone)]
struct FileAnalysis {
    path: PathBuf,
    word_count: usize,
    bag_of_words: HashMap<String, usize>,
    last_modified: SystemTime,
}

/// Represents the analysis of an entire project.
#[derive(Debug, Serialize, Deserialize, Clone)]
struct ProjectAnalysis {
    project_root: PathBuf,
    rust_files: Vec<FileAnalysis>,
}

/// Represents the similarity between two files.
#[derive(Debug, Serialize, Deserialize, Clone)]
struct FilePairSimilarity {
    file1_path: PathBuf,
    file2_path: PathBuf,
    similarity: f64,
}

// Define a list of common English stopwords
const STOPWORDS: &[&str] = &[
    "a", "an", "and", "are", "as", "at", "be", "by", "for", "from", "has", "he", "in",
    "is", "it", "its", "of", "on", "that", "the", "to", "was", "were", "will", "with",
];

/// Tokenizes a string into a vector of words, filtering out stopwords.
fn tokenize(text: &str) -> Vec<String> {
    puffin::profile_function!();
    let re = Regex::new(r"\b\w+\b").unwrap();
    re.find_iter(text)
        .map(|m| m.as_str().to_lowercase())
        .filter(|word| !STOPWORDS.contains(&word.as_str())) // Filter out stopwords
        .collect()
}

/// Calculates the cosine similarity between two bags of words.
fn calculate_cosine_similarity(map1: &HashMap<String, usize>, map2: &HashMap<String, usize>) -> f64 {
    puffin::profile_function!();
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

/// Runs the full analysis of all Rust projects in the specified root directory.
fn run_full_analysis() -> Result<()> {
    puffin::profile_function!();
    // Set up Rayon thread pool
    rayon::ThreadPoolBuilder::new().num_threads(16).build_global().unwrap();

    let search_root = PathBuf::from(GITHUB_ROOT_DIR);
    let term_index_file = search_root.join("term_index.json");
    let file_pair_similarities_file = search_root.join("file_pair_similarities.json");

    let mut all_rust_files: Vec<FileAnalysis> = Vec::new();

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

    // Second pass: Process .rs files within identified project roots
    for project_root in discovered_project_roots {
        puffin::profile_scope!("process_project", project_root.to_str().unwrap_or_default());
        let project_summary_file = project_root.join(".file_analysis_summary.json");
        let cargo_toml_path = project_root.join("Cargo.toml");

        let mut should_reprocess_project = true;

        // Check if project summary file exists and is newer than Cargo.toml
        if project_summary_file.exists() {
            if let Ok(summary_metadata) = fs::metadata(&project_summary_file) {
                if let Ok(summary_mtime) = summary_metadata.modified() {
                    if let Ok(cargo_metadata) = fs::metadata(&cargo_toml_path) {
                        if let Ok(cargo_mtime) = cargo_metadata.modified() {
                            if summary_mtime >= cargo_mtime {
                                // Summary file is up-to-date or newer than Cargo.toml
                                // Now check individual file staleness
                                if let Ok(cached_data) = fs::read_to_string(&project_summary_file) {
                                    if let Ok(cached_project_analysis) = serde_json::from_str::<ProjectAnalysis>(&cached_data) {
                                        let mut project_is_stale = false;
                                        for file_analysis in &cached_project_analysis.rust_files {
                                            if let Ok(metadata) = fs::metadata(&file_analysis.path) {
                                                if let Ok(modified_time) = metadata.modified() {
                                                    if modified_time > file_analysis.last_modified {
                                                        project_is_stale = true;
                                                        eprintln!("  File {:?} is newer than cached. Project {:?} is stale.", file_analysis.path, project_root);
                                                        break;
                                                    }
                                                }
                                            } else {
                                                // File is missing or inaccessible, consider project stale
                                                project_is_stale = true;
                                                eprintln!("  File {:?} is missing or inaccessible. Project {:?} is stale.", file_analysis.path, project_root);
                                                break;
                                            }
                                        }
                                        if !project_is_stale {
                                            should_reprocess_project = false;
                                            eprintln!("Skipping up-to-date project: {:?}", project_root);
                                            // Add files from this project to all_rust_files
                                            all_rust_files.extend(cached_project_analysis.rust_files.into_iter());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if should_reprocess_project {
            eprintln!("Processing project: {:?}", project_root);
            let mut current_project_rust_files: Vec<FileAnalysis> = Vec::new();

            let mut prioritized_files: std::collections::HashSet<PathBuf> = std::collections::HashSet::new();

            // Parse Cargo.toml to find prioritized files
            if let Ok(cargo_toml_content) = fs::read_to_string(&cargo_toml_path) {
                if let Ok(parsed_toml) = cargo_toml_content.parse::<Value>() {
                    // Add lib.rs if it exists
                    if let Some(lib_table) = parsed_toml.get("lib") {
                        if let Some(lib_path) = lib_table.get("path").and_then(|path| path.as_str()) {
                            prioritized_files.insert(project_root.join(lib_path));
                        } else {
                            prioritized_files.insert(project_root.join("src/lib.rs"));
                        }
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
            let processed_prioritized_files: Vec<FileAnalysis> = prioritized_files.par_iter().filter_map(|path| {
                if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
                    match fs::read_to_string(path) {
                        Ok(file_content) => {
                            let tokens = tokenize(&file_content);
                            let mut bag_of_words: HashMap<String, usize> = HashMap::new();
                            for token in &tokens {
                                *bag_of_words.entry(token.clone()).or_insert(0) += 1;
                            }
                            let word_count = tokens.len();
                            let last_modified = fs::metadata(path).and_then(|m| m.modified()).unwrap_or_else(|_| SystemTime::now());

                            Some(FileAnalysis {
                                path: path.to_path_buf(),
                                word_count,
                                bag_of_words,
                                last_modified,
                            })
                        },
                        Err(e) => {
                            eprintln!("  Error reading prioritized Rust file {:?}: {}", path, e);
                            None
                        }
                    }
                } else {
                    None
                }
            }).collect();
            current_project_rust_files.extend(processed_prioritized_files.into_iter());

            // Process remaining .rs files in parallel
            let remaining_files: Vec<FileAnalysis> = WalkDir::new(&project_root)
                .into_iter()
                .filter_map(|e| e.ok())
                .par_bridge() // Use par_bridge for parallel iteration over WalkDir entries
                .filter(|e| !e.path().components().any(|comp| comp.as_os_str() == "target")) // Skip target directories
                .filter_map(|entry| {
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
                                let last_modified = fs::metadata(path).and_then(|m| m.modified()).unwrap_or_else(|_| SystemTime::now());

                                Some(FileAnalysis {
                                    path: path.to_path_buf(),
                                    word_count,
                                    bag_of_words,
                                    last_modified,
                                })
                            },
                            Err(e) => {
                                eprintln!("  Error reading Rust file {:?}: {}", path, e);
                                None
                            }
                        }
                    } else {
                        None
                    }
                }).collect();
            current_project_rust_files.extend(remaining_files.into_iter());

            let project_analysis = ProjectAnalysis {
                project_root: project_root.clone(),
                rust_files: current_project_rust_files.clone(),
            };
            // Save per-project summary
            eprintln!("Saving project summary to {:?}", project_summary_file);
            let serialized = serde_json::to_string_pretty(&project_analysis)?;
            fs::write(&project_summary_file, serialized)?;

            all_rust_files.extend(current_project_rust_files.into_iter());
        }
    }

    eprintln!("All project analysis complete. Finalizing similarity calculation...");

    // File similarity calculation and storage
    let mut file_pair_similarities: Vec<FilePairSimilarity> = Vec::new();
    eprintln!("Calculating and storing file-level similarities...");
    for i in 0..all_rust_files.len() {
        for j in (i + 1)..all_rust_files.len() {
            let file1 = &all_rust_files[i];
            let file2 = &all_rust_files[j];

            let similarity = calculate_cosine_similarity(&file1.bag_of_words, &file2.bag_of_words);

            if similarity >= 0.90 {
                file_pair_similarities.push(FilePairSimilarity {
                    file1_path: file1.path.clone(),
                    file2_path: file2.path.clone(),
                    similarity,
                });
            }
        }
    }

    eprintln!("Saving file pair similarities to {:?}", file_pair_similarities_file);
    let serialized_file_pair_similarities = serde_json::to_string_pretty(&file_pair_similarities)?;
    fs::write(&file_pair_similarities_file, serialized_file_pair_similarities)?;

    eprintln!("Similarity calculation complete.");

    // Build and save term index
    let mut term_index: HashMap<String, HashSet<String>> = HashMap::new();
    for file_analysis in &all_rust_files {
        let crate_name = if let Ok(cargo_toml_content) = fs::read_to_string(file_analysis.path.parent().unwrap().join("Cargo.toml")) {
            if let Ok(parsed_toml) = cargo_toml_content.parse::<Value>() {
                if let Some(package_table) = parsed_toml.get("package") {
                    if let Some(name) = package_table.get("name").and_then(|name| name.as_str()) {
                        name.to_string()
                    } else {
                        file_analysis.path.parent().unwrap().file_name().unwrap_or_default().to_string_lossy().into_owned()
                    }
                } else {
                    file_analysis.path.parent().unwrap().file_name().unwrap_or_default().to_string_lossy().into_owned()
                }
            } else {
                file_analysis.path.parent().unwrap().file_name().unwrap_or_default().to_string_lossy().into_owned()
            }
        } else {
            file_analysis.path.parent().unwrap().file_name().unwrap_or_default().to_string_lossy().into_owned()
        };

        for (word, _count) in &file_analysis.bag_of_words {
            term_index.entry(word.clone()).or_insert_with(HashSet::new).insert(crate_name.clone());
        }
    }

    eprintln!("Saving term index to {:?}", term_index_file);
    let serialized_term_index = serde_json::to_string_pretty(&term_index)?;
    fs::write(&term_index_file, serialized_term_index)?;

    Ok(())
}

/// Reads all Cargo.toml files in the specified root directory and prints their content.
fn run_read_cargo_toml_mode() -> Result<()> {
    puffin::profile_function!();
    let search_root = PathBuf::from(GITHUB_ROOT_DIR);

    eprintln!("Searching for Cargo.toml files in: {:?}", search_root);

    for entry in WalkDir::new(&search_root)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() && path.file_name().map_or(false, |name| name == "Cargo.toml") {
            eprintln!("Found Cargo.toml: {:?}", path);
            match fs::read_to_string(path) {
                Ok(content) => {
                    println!("\n--- Content of {:?} ---\n{}\n", path, content);
                },
                Err(e) => {
                    eprintln!("Error reading Cargo.toml {:?}: {}", path, e);
                }
            }
        }
    }

    Ok(())
}

/// Runs the crate similarity analysis.
fn run_crate_similarity_analysis(target_crate_name: Option<String>, num_results: usize) -> Result<()> {
    puffin::profile_function!();
    let search_root = PathBuf::from(GITHUB_ROOT_DIR);
    let file_pair_similarities_file = search_root.join("file_pair_similarities.json");

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

    let mut all_project_analyses: Vec<ProjectAnalysis> = Vec::new();

    // Load project summaries
    for project_root in discovered_project_roots {
        let project_summary_file = project_root.join(".file_analysis_summary.json");
        if project_summary_file.exists() {
            if let Ok(cached_data) = fs::read_to_string(&project_summary_file) {
                if let Ok(project_analysis) = serde_json::from_str::<ProjectAnalysis>(&cached_data) {
                    all_project_analyses.push(project_analysis);
                }
            }
        }
    }

    let mut crate_bags_of_words: HashMap<String, HashMap<String, usize>> = HashMap::new();
    let mut crate_file_paths: HashMap<String, HashSet<PathBuf>> = HashMap::new();

    for project_analysis in &all_project_analyses {
        let mut aggregated_bag_of_words: HashMap<String, usize> = HashMap::new();
        let mut current_crate_file_paths: HashSet<PathBuf> = HashSet::new();

        for file_analysis in &project_analysis.rust_files {
            for (word, count) in &file_analysis.bag_of_words {
                *aggregated_bag_of_words.entry(word.clone()).or_insert(0) += count;
            }
            current_crate_file_paths.insert(file_analysis.path.clone());
        }
        // Extract crate name from Cargo.toml
        let cargo_toml_path = project_analysis.project_root.join("Cargo.toml");
        let crate_name = if let Ok(cargo_toml_content) = fs::read_to_string(&cargo_toml_path) {
            if let Ok(parsed_toml) = cargo_toml_content.parse::<Value>() {
                if let Some(package_table) = parsed_toml.get("package") {
                    if let Some(name) = package_table.get("name").and_then(|name| name.as_str()) {
                        name.to_string()
                    } else {
                        project_analysis.project_root.file_name().unwrap_or_default().to_string_lossy().into_owned()
                    }
                } else {
                    project_analysis.project_root.file_name().unwrap_or_default().to_string_lossy().into_owned()
                }
            } else {
                project_analysis.project_root.file_name().unwrap_or_default().to_string_lossy().into_owned()
            }
        } else {
            project_analysis.project_root.file_name().unwrap_or_default().to_string_lossy().into_owned()
        };
        crate_bags_of_words.insert(crate_name.clone(), aggregated_bag_of_words);
        crate_file_paths.insert(crate_name, current_crate_file_paths);
    }

    let target_crate_name_str = target_crate_name.unwrap_or_else(|| "file_content_analyzer".to_string());
    let target_crate_bag = crate_bags_of_words.get(&target_crate_name_str).ok_or(anyhow::anyhow!("Target crate '{}' not found in cache. Run full_analysis first.", target_crate_name_str))?;

    eprintln!("Calculating similarities to '{}'ները...", target_crate_name_str);
    let mut similarities: Vec<(String, f64)> = Vec::new();
    for (crate_name, bag_of_words) in &crate_bags_of_words {
        if crate_name != &target_crate_name_str {
            let similarity = calculate_cosine_similarity(target_crate_bag, bag_of_words);
            similarities.push((crate_name.clone(), similarity));
        }
    }

    similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    println!("\n--- Top {} Similar Crates to {} ---", num_results, target_crate_name_str);
    let top_similar_crates: Vec<String> = similarities.iter().take(num_results).map(|(name, _)| name.clone()).collect();

    for (crate_name, similarity) in similarities.iter().take(num_results) {
        println!("Crate: {}, Similarity: {:.2}%", crate_name, similarity * 100.0);
    }

    // Load file pair similarities
    if file_pair_similarities_file.exists() {
        eprintln!("Loading file pair similarities from {:?}", file_pair_similarities_file);
        let cached_data = fs::read_to_string(&file_pair_similarities_file)?;
        let all_file_pair_similarities: Vec<FilePairSimilarity> = serde_json::from_str(&cached_data)?;

        println!("\n--- File-level Similarities within Top Similar Crates ---");
        let target_crate_files = crate_file_paths.get(&target_crate_name_str).cloned().unwrap_or_default();

        for similar_crate_name in &top_similar_crates {
            if let Some(similar_crate_files) = crate_file_paths.get(similar_crate_name) {
                println!("\n  Files similar to {} (in {})", target_crate_name_str, similar_crate_name);
                let mut found_similarities = false;
                for file_pair in &all_file_pair_similarities {
                    let file1_in_target = target_crate_files.contains(&file_pair.file1_path);
                    let file2_in_target = target_crate_files.contains(&file_pair.file2_path);
                    let file1_in_similar = similar_crate_files.contains(&file_pair.file1_path);
                    let file2_in_similar = similar_crate_files.contains(&file_pair.file2_path);

                    // Case 1: One file from target crate, one from similar crate
                    if (file1_in_target && file2_in_similar) || (file2_in_target && file1_in_similar) {
                        println!("    File 1: {:?}", file_pair.file1_path);
                        println!("    File 2: {:?}", file_pair.file2_path);
                        println!("    Similarity: {:.2}%", file_pair.similarity * 100.0);
                        found_similarities = true;
                    }
                }
                if !found_similarities {
                    println!("    No file-level similarities found between these crates (>=90%).");
                }
            }
        }
    } else {
        eprintln!("File pair similarities file {:?} not found. Run full_analysis first.", file_pair_similarities_file);
    }

    Ok(())
}

/// Searches for keywords in the hierarchical term index.
fn run_search_keywords_mode(keywords: Vec<String>, search_path: Option<PathBuf>) -> Result<()> {
    puffin::profile_function!();
    let search_root = PathBuf::from(GITHUB_ROOT_DIR);
    let hierarchical_term_index_file = search_root.join("hierarchical_term_index.json");

    eprintln!("Loading hierarchical term index from {:?}", hierarchical_term_index_file);
    let cached_data = fs::read_to_string(&hierarchical_term_index_file)?;
    let hierarchical_term_index: HashMap<String, HashMap<PathBuf, usize>> = serde_json::from_str(&cached_data)?;

    if keywords.is_empty() {
        eprintln!("No keywords provided for search.");
        return Ok(())
    }

    let mut matching_files_summary: HashMap<PathBuf, usize> = HashMap::new(); // File path -> count of matching keywords

    println!("\n--- Detailed Search Results for Keywords {:?} ---", keywords);
    if let Some(ref path_filter) = search_path {
        println!("  (Filtered by path: {:?})", path_filter);
    }

    for keyword in &keywords {
        println!("\nKeyword: '{}'\n--------------------", keyword);
        if let Some(file_counts) = hierarchical_term_index.get(&keyword.to_lowercase()) {
            let mut files_for_keyword: Vec<(&PathBuf, &usize)> = file_counts.iter().collect();
            files_for_keyword.sort_by(|a, b| b.1.cmp(a.1)); // Sort by count, descending

            if files_for_keyword.is_empty() {
                println!("  No files found for this keyword.");
            } else {
                for (file_path, count) in files_for_keyword {
                    // Filter by search_path if provided
                    if let Some(ref path_filter) = search_path {
                        if !file_path.starts_with(path_filter) {
                            continue;
                        }
                    }
                    println!("  File: {:?}, Occurrences: {}", file_path, count);
                    *matching_files_summary.entry(file_path.clone()).or_insert(0) += 1;
                }
            }
        } else {
            println!("  Keyword not found in index.");
        }
    }

    println!("\n--- Summary of Matching Files ---");
    let mut sorted_summary: Vec<(PathBuf, usize)> = matching_files_summary.into_iter().collect();
    sorted_summary.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by number of matching keywords, descending

    if sorted_summary.is_empty() {
        println!("No files found matching any of the provided keywords.");
    } else {
        for (file_path, match_count) in sorted_summary {
            println!("File: {:?}, Matching Keywords: {}", file_path, match_count);
        }
    }

    Ok(())
}

/// Generates a report of stopword candidates.
fn run_generate_stopword_report_mode() -> Result<()> {
    puffin::profile_function!();
    let search_root = PathBuf::from(GITHUB_ROOT_DIR);

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

    let mut all_rust_files: Vec<FileAnalysis> = Vec::new();

    // Load project summaries
    for project_root in discovered_project_roots {
        let project_summary_file = project_root.join(".file_analysis_summary.json");
        if project_summary_file.exists() {
            if let Ok(cached_data) = fs::read_to_string(&project_summary_file) {
                if let Ok(project_analysis) = serde_json::from_str::<ProjectAnalysis>(&cached_data) {
                    all_rust_files.extend(project_analysis.rust_files.into_iter());
                }
            }
        }
    }

    if all_rust_files.is_empty() {
        eprintln!("No Rust files found in cache. Run full_analysis first.");
        return Ok(())
    }

    let total_files = all_rust_files.len();
    let mut word_file_counts: HashMap<String, usize> = HashMap::new();

    for file_analysis in &all_rust_files {
        for (word, _count) in &file_analysis.bag_of_words {
            *word_file_counts.entry(word.clone()).or_insert(0) += 1;
        }
    }

    let mut stopword_candidates: Vec<(String, f64)> = Vec::new();
    for (word, count) in word_file_counts {
        let percentage = (count as f64 / total_files as f64) * 100.0;
        stopword_candidates.push((word, percentage));
    }

    stopword_candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    println!("\n--- Stopword Candidates (by percentage of files appeared in) ---");
    if stopword_candidates.is_empty() {
        println!("No stopword candidates found.");
    } else {
        for (word, percentage) in stopword_candidates.iter().take(100) { // Print top 100
            println!("Word: '{}', Appears in: {:.2}% of files", word, percentage);
        }
        eprintln!("Full stopword report saved to full_stopword_report.json");
        let serialized_report = serde_json::to_string_pretty(&stopword_candidates)?;
        fs::write("full_stopword_report.json", serialized_report)?;
    }

    Ok(())
}

/// Migrates the old cache file to the new per-project summary format.
fn run_migrate_cache_mode() -> Result<()> {
    puffin::profile_function!();
    let search_root = PathBuf::from(GITHUB_ROOT_DIR);
    let old_cache_file = search_root.join("file_analysis_cache.json");

    eprintln!("Attempting to migrate old cache from {:?}", old_cache_file);

    if old_cache_file.exists() {
        let cached_data = fs::read_to_string(&old_cache_file)?;
        let all_project_analyses: Vec<ProjectAnalysis> = serde_json::from_str(&cached_data)?;

        for project_analysis in all_project_analyses {
            let project_summary_file = project_analysis.project_root.join(".file_analysis_summary.json");
            eprintln!("Migrating project {:?} to {:?}", project_analysis.project_root, project_summary_file);
            let serialized = serde_json::to_string_pretty(&project_analysis)?;
            fs::write(&project_summary_file, serialized)?;
        }
        eprintln!("Migration complete. You can now safely remove {:?}", old_cache_file);
    } else {
        eprintln!("Old cache file {:?} not found. Nothing to migrate.", old_cache_file);
    }

    Ok(())
}

/// Builds a hierarchical term index for the entire codebase.
fn build_directory_index(current_dir: &Path, search_root: &Path) -> Result<HashMap<String, HashMap<PathBuf, usize>>> {
    puffin::profile_function!();
    let mut local_index: HashMap<String, HashMap<PathBuf, usize>> = HashMap::new();
    let dir_summary_file = current_dir.join(".dir_index.json");

    // Check if a cached summary exists and is up-to-date
    if dir_summary_file.exists() {
        if let Ok(metadata) = fs::metadata(&dir_summary_file) {
            if let Ok(modified_time) = metadata.modified() {
                let mut all_files_up_to_date = true;
                // Check if any file in current_dir or its immediate subdirs is newer than the summary
                for entry in WalkDir::new(current_dir).max_depth(1).into_iter().filter_map(|e| e.ok()) {
                    let path = entry.path();
                    if path.is_file() {
                        if let Ok(file_modified_time) = fs::metadata(path).and_then(|m| m.modified()) {
                            if file_modified_time > modified_time {
                                all_files_up_to_date = false;
                                break;
                            }
                        }
                    }
                }
                if all_files_up_to_date {
                    // Load from cache
                    if let Ok(cached_data) = fs::read_to_string(&dir_summary_file) {
                        if let Ok(index) = serde_json::from_str::<HashMap<String, HashMap<PathBuf, usize>>>(&cached_data) {
                            eprintln!("Loaded cached index for: {:?}", current_dir);
                            return Ok(index);
                        }
                    }
                }
            }
        }
    }

    eprintln!("Processing directory: {:?}", current_dir);

    // Process files directly in the current directory
    for entry in WalkDir::new(current_dir).max_depth(1).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            if let Ok(file_content) = fs::read_to_string(path) {
                let tokens = tokenize(&file_content);
                let mut bag_of_words: HashMap<String, usize> = HashMap::new();
                for token in &tokens {
                    *bag_of_words.entry(token.clone()).or_insert(0) += 1;
                }

                let relative_path = path.strip_prefix(search_root)
                    .unwrap_or(path)
                    .to_path_buf();

                for (word, count) in bag_of_words {
                    local_index
                        .entry(word)
                        .or_insert_with(HashMap::new)
                        .insert(relative_path.clone(), count);
                }
            }
        }
    }

    // Process subdirectories in parallel and merge their indexes
    let sub_dir_indexes: Vec<HashMap<String, HashMap<PathBuf, usize>>> = WalkDir::new(current_dir)
        .min_depth(1) // Start from subdirectories
        .max_depth(1) // Only immediate subdirectories
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .collect::<Vec<_>>() // Collect into a Vec first
        .par_iter() // Process subdirectories in parallel
        .filter_map(|entry| {
            let sub_dir_path = entry.path();
            // Skip target directories
            if sub_dir_path.components().any(|comp| comp.as_os_str() == "target") {
                return None;
            }
            match build_directory_index(sub_dir_path, search_root) {
                Ok(index) => Some(index),
                Err(e) => {
                    eprintln!("Error processing subdirectory {:?}: {}", sub_dir_path, e);
                    None
                }
            }
        })
        .collect();

    for sub_index in sub_dir_indexes {
        for (word, file_counts) in sub_index {
            for (path, count) in file_counts {
                local_index
                    .entry(word.clone()) // Clone the word here
                    .or_insert_with(HashMap::new)
                    .insert(path, count);
            }
        }
    }

    // Save the local index for this directory
    eprintln!("Saving directory index for: {:?}", current_dir);
    let serialized_local_index = serde_json::to_string_pretty(&local_index)?;
    fs::write(&dir_summary_file, serialized_local_index)?;

    Ok(local_index)
}

/// Builds a hierarchical term index for the entire codebase.
fn run_build_hierarchical_index_mode() -> Result<()> {
    puffin::profile_function!();
    // Set up Rayon thread pool
    rayon::ThreadPoolBuilder::new().num_threads(16).build_global().unwrap();

    let search_root = PathBuf::from(GITHUB_ROOT_DIR);
    let hierarchical_term_index_file = search_root.join("hierarchical_term_index.json");

    eprintln!("Starting hierarchical index build from: {:?}", search_root);

    let final_index = build_directory_index(&search_root, &search_root)?;

    eprintln!("Saving final hierarchical term index to {:?}. Total unique terms: {}", hierarchical_term_index_file, final_index.len());
    let serialized_final_index = serde_json::to_string_pretty(&final_index)?;
    fs::write(&hierarchical_term_index_file, serialized_final_index)?;

    Ok(())
}


/// Finds terms that are shared across the exact same set of files.
fn run_find_exact_shared_files_terms_mode(search_path: Option<PathBuf>, filter_by_path: Option<PathBuf>) -> Result<()> {
    puffin::profile_function!();
    let search_root = PathBuf::from(GITHUB_ROOT_DIR);
    let hierarchical_term_index_file = search_root.join("hierarchical_term_index.json");

    eprintln!("Loading hierarchical term index from {:?}", hierarchical_term_index_file);
    let cached_data = fs::read_to_string(&hierarchical_term_index_file)?;
    let hierarchical_term_index: HashMap<String, HashMap<PathBuf, usize>> = serde_json::from_str(&cached_data)?;

    let files_to_terms: HashMap<Vec<PathBuf>, HashSet<String>> = hierarchical_term_index.into_par_iter()
        .filter_map(|(term, file_counts)| {
            let mut file_paths_vec: Vec<PathBuf> = file_counts.keys()
                .filter(|file_path| {
                    if let Some(ref path_filter) = search_path {
                        file_path.starts_with(path_filter)
                    } else {
                        true
                    }
                })
                .cloned()
                .collect();
            file_paths_vec.sort(); // Sort to ensure consistent order for hashing

            if !file_paths_vec.is_empty() {
                Some((file_paths_vec, term))
            } else {
                None
            }
        })
        .fold(HashMap::new, |mut acc: HashMap<Vec<PathBuf>, HashSet<String>>, (file_paths_vec, term)| {
            acc.entry(file_paths_vec).or_insert_with(HashSet::new).insert(term);
            acc
        })
        .reduce(HashMap::new, |mut map1, map2| {
            for (key, value) in map2 {
                map1.entry(key).or_insert_with(HashSet::new).extend(value);
            }
            map1
        });

    #[derive(Serialize)]
    struct SharedTermsGroup {
        group_id: usize,
        files: Vec<PathBuf>,
        terms: Vec<String>,
    }

    let mut groups: Vec<SharedTermsGroup> = Vec::new();
    let mut count = 0;

    for (file_paths, terms) in files_to_terms {
        // Apply filter_by_path if provided
        let passes_filter = if let Some(ref path_filter) = filter_by_path {
            file_paths.iter().any(|p| p.starts_with(path_filter))
        } else {
            true // No filter applied
        };

        if terms.len() > 1 && passes_filter {
            count += 1;
            groups.push(SharedTermsGroup {
                group_id: count,
                files: file_paths,
                terms: terms.into_iter().collect(), // Convert HashSet to Vec
            });
        }
    }

    // Print JSON output to stdout
    println!("{}", serde_json::to_string_pretty(&groups)?);

    if count == 0 {
        eprintln!("No terms found sharing the exact same set of files (excluding single-term groups).");
    }

    Ok(())
}

fn run_estimate_mode() -> Result<()> {
    puffin::profile_function!();
    let search_root = PathBuf::from(GITHUB_ROOT_DIR);
    let mut new_files = Vec::new();
    let mut modified_files = Vec::new();
    let mut deleted_files = Vec::new();
    let mut total_size = 0;

    eprintln!("Estimating work to be done...");

    let mut discovered_project_roots: std::collections::HashSet<PathBuf> = std::collections::HashSet::new();
    {
        puffin::profile_scope!("discover_projects");
        for entry in WalkDir::new(&search_root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| !e.path().components().any(|comp| comp.as_os_str() == "target"))
        {
            let path = entry.path();
            if path.is_file() && path.file_name().map_or(false, |name| name == "Cargo.toml") {
                if let Some(parent) = path.parent() {
                    discovered_project_roots.insert(parent.to_path_buf());
                }
            }
        }
    }

    for project_root in discovered_project_roots {
        puffin::profile_scope!("estimate_project", project_root.to_str().unwrap_or_default());
        let project_summary_file = project_root.join(".file_analysis_summary.json");

        if project_summary_file.exists() {
            if let Ok(cached_data) = fs::read_to_string(&project_summary_file) {
                if let Ok(cached_project_analysis) = serde_json::from_str::<ProjectAnalysis>(&cached_data) {
                    let mut cached_files: HashMap<PathBuf, FileAnalysis> = cached_project_analysis.rust_files.into_iter().map(|f| (f.path.clone(), f)).collect();

                    for entry in WalkDir::new(&project_root)
                        .into_iter()
                        .filter_map(|e| e.ok())
                        .filter(|e| !e.path().components().any(|comp| comp.as_os_str() == "target"))
                        .filter(|e| e.path().is_file() && e.path().extension().map_or(false, |ext| ext == "rs"))
                    {
                        let path = entry.path();
                        if let Some(cached_file) = cached_files.remove(path) {
                            if let Ok(metadata) = fs::metadata(path) {
                                if let Ok(modified_time) = metadata.modified() {
                                    if modified_time > cached_file.last_modified {
                                        modified_files.push(path.to_path_buf());
                                        total_size += metadata.len();
                                    }
                                }
                            }
                        } else {
                            new_files.push(path.to_path_buf());
                            if let Ok(metadata) = fs::metadata(path) {
                                total_size += metadata.len();
                            }
                        }
                    }

                    deleted_files.extend(cached_files.keys().cloned());
                }
            }
        } else {
            for entry in WalkDir::new(&project_root)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| !e.path().components().any(|comp| comp.as_os_str() == "target"))
                .filter(|e| e.path().is_file() && e.path().extension().map_or(false, |ext| ext == "rs"))
            {
                new_files.push(entry.path().to_path_buf());
                if let Ok(metadata) = fs::metadata(entry.path()) {
                    total_size += metadata.len();
                }
            }
        }
    }

    println!("--- Indexing Estimation Report ---");
    println!("New files: {}", new_files.len());
    for file in new_files {
        println!("  - {:?}", file);
    }
    println!("Modified files: {}", modified_files.len());
    for file in modified_files {
        println!("  - {:?}", file);
    }
    println!("Deleted files: {}", deleted_files.len());
    for file in deleted_files {
        println!("  - {:?}", file);
    }

    // Simple estimation heuristic: 1 second per 100KB
    let estimated_time_seconds = (total_size as f64 / 102400.0).ceil();
    println!("\nEstimated time to index: {} seconds", estimated_time_seconds);

    Ok(())
}


/// Main entry point for the application.
fn main() -> Result<()> {
    let args = Args::parse();

    if args.profile {
//  #[cfg(feature = "profile")]
    puffin::set_scopes_on(true);
    }

    let result = match args.mode.as_str() {
        "full_analysis" => run_full_analysis(),
        "read_cargo_toml" => run_read_cargo_toml_mode(),
        "crate_similarity" => run_crate_similarity_analysis(args.target_crate, args.most_similar),
        "migrate_cache" => run_migrate_cache_mode(),
        "search_keywords" => run_search_keywords_mode(args.keywords, args.search_path),
        "generate_stopword_report" => run_generate_stopword_report_mode(),
        "build_hierarchical_index" => run_build_hierarchical_index_mode(),
        "find_exact_shared_files_terms" => run_find_exact_shared_files_terms_mode(args.search_path, args.filter_by_path),
        "estimate" => run_estimate_mode(),
        _ => Err(anyhow::anyhow!("Invalid mode specified. Use 'full_analysis', 'read_cargo_toml', 'crate_similarity', 'migrate_cache', 'search_keywords', 'generate_stopword_report', 'build_hierarchical_index', 'find_exact_shared_files_terms', or 'estimate'.")),
    };

//o    #[cfg(feature = "profile")]
    // if args.profile {
    //     if let Ok(guard) = puffin_egui::start_puffin_server() {
    //         eprintln!("Started puffin server on {}", guard.url());
    //         // Keep the server running until the user presses enter
    //         let mut input = String::new();
    //         std::io::stdin().read_line(&mut input)?;
    //     }
    // }

    result
}
