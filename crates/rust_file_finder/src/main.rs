use walkdir::WalkDir;
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use regex::Regex;
use toml::Value;
use clap::Parser;
use rayon::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};
use anyhow::Result;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Mode of operation: 'full_analysis', 'read_cargo_toml', 'crate_similarity', 'migrate_cache', 'search_keywords', or 'generate_stopword_report'
    #[arg(short, long, default_value = "full_analysis")]
    mode: String,

    /// Target crate for similarity analysis (used with --mode crate_similarity)
    #[arg(long)]
    target_crate: Option<String>,

    /// Number of most similar crates to display (used with --mode crate_similarity)
    #[arg(long, default_value_t = 10)]
    most_similar: usize,

    /// Keywords to search for (used with --mode search_keywords)
    #[arg(long, value_delimiter = ' ')]
    keywords: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct FileAnalysis {
    path: PathBuf,
    word_count: usize,
    bag_of_words: HashMap<String, usize>,
    last_modified: SystemTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ProjectAnalysis {
    project_root: PathBuf,
    rust_files: Vec<FileAnalysis>,
}

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

fn tokenize(text: &str) -> Vec<String> {
    let re = Regex::new(r"\b\w+\b").unwrap();
    re.find_iter(text)
        .map(|m| m.as_str().to_lowercase())
        .filter(|word| !STOPWORDS.contains(&word.as_str())) // Filter out stopwords
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

fn run_full_analysis() -> Result<()> {
    // Set up Rayon thread pool
    rayon::ThreadPoolBuilder::new().num_threads(16).build_global().unwrap();

    let search_root = PathBuf::from("/data/data/com.termux/files/home/storage/github/");
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

fn run_read_cargo_toml_mode() -> Result<()> {
    let search_root = PathBuf::from("/data/data/com.termux/files/home/storage/github/");

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
                    println!("\n--- Content of {:?} ---\n{}", path, content);
                },
                Err(e) => {
                    eprintln!("Error reading Cargo.toml {:?}: {}", path, e);
                }
            }
        }
    }

    Ok(())
}

fn run_crate_similarity_analysis(target_crate_name: Option<String>, num_results: usize) -> Result<()> {
    let search_root = PathBuf::from("/data/data/com.termux/files/home/storage/github/");
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

    eprintln!("Calculating similarities to '{}'...", target_crate_name_str);
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

fn run_search_keywords_mode(keywords: Vec<String>) -> Result<()> {
    let search_root = PathBuf::from("/data/data/com.termux/files/home/storage/github/");
    let term_index_file = search_root.join("term_index.json");

    eprintln!("Loading term index from {:?}", term_index_file);
    let cached_data = fs::read_to_string(&term_index_file)?;
    let term_index: HashMap<String, HashSet<String>> = serde_json::from_str(&cached_data)?;

    if keywords.is_empty() {
        eprintln!("No keywords provided for search.");
        return Ok!(());
    }

    let mut matching_crates: HashMap<String, usize> = HashMap::new(); // Crate name -> count of matching keywords

    eprintln!("Searching for keywords: {:?}", keywords);
    for keyword in keywords {
        if let Some(crates) = term_index.get(&keyword.to_lowercase()) {
            for crate_name in crates {
                *matching_crates.entry(crate_name.clone()).or_insert(0) += 1;
            }
        }
    }

    let mut sorted_results: Vec<(String, usize)> = matching_crates.into_iter().collect();
    sorted_results.sort_by(|a, b| b.1.cmp(&a.1));

    println!("\n--- Search Results for Keywords {:?} ---", keywords);
    if sorted_results.is_empty() {
        println!("No crates found matching the provided keywords.");
    } else {
        for (crate_name, match_count) in sorted_results {
            println!("Crate: {}, Matching Keywords: {}", crate_name, match_count);
        }
    }

    Ok(())
}

fn run_migrate_cache_mode() -> Result<()> {
    let search_root = PathBuf::from("/data/data/com.termux/files/home/storage/github/");
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

fn main() -> Result<()> {
    let args = Args::parse();

    match args.mode.as_str() {
        "full_analysis" => run_full_analysis(),
        "read_cargo_toml" => run_read_cargo_toml_mode(),
        "crate_similarity" => run_crate_similarity_analysis(args.target_crate, args.most_similar),
        "migrate_cache" => run_migrate_cache_mode(),
        "search_keywords" => run_search_keywords_mode(args.keywords),
        _ => Err(anyhow::anyhow!("Invalid mode specified. Use 'full_analysis', 'read_cargo_toml', 'crate_similarity', 'migrate_cache', or 'search_keywords'.")),
    }
}
