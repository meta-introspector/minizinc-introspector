use crate::utils::error::Result;
use crate::cli::AnalyzeDuplicatesArgs;
use std::fs;
use std::path::PathBuf; // Removed Path
use walkdir::WalkDir;
use syn::{self, visit};
use std::collections::HashMap; // Added HashMap import

use std::collections::HashSet; // Added HashSet import
use quote::ToTokens; // Added ToTokens import
use syn::visit::Visit; // Added Visit trait import

// A visitor to collect code features
struct CodeFeatureExtractor {
    #[allow(dead_code)]
    file_path: PathBuf,
    keywords: HashSet<String>,
    fn_count: usize,
    struct_count: usize,
    enum_count: usize,
    trait_count: usize,
    mod_count: usize,
    // Add other AST-related metrics here
}

impl CodeFeatureExtractor {
    fn new(file_path: PathBuf) -> Self {
        CodeFeatureExtractor {
            file_path,
            keywords: HashSet::new(),
            fn_count: 0,
            struct_count: 0,
            enum_count: 0,
            trait_count: 0,
            mod_count: 0,
        }
    }

    fn add_keyword(&mut self, keyword: String) {
        self.keywords.insert(keyword);
    }
}

impl<'ast> visit::Visit<'ast> for CodeFeatureExtractor { // Changed to visit::Visit
    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        self.fn_count += 1;
        self.add_keyword(i.sig.ident.to_string());
        // Recursively visit children to find more keywords/identifiers
        visit::visit_item_fn(self, i);
    }

    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        self.struct_count += 1;
        self.add_keyword(i.ident.to_string());
        visit::visit_item_struct(self, i);
    }

    fn visit_item_enum(&mut self, i: &'ast syn::ItemEnum) {
        self.enum_count += 1;
        self.add_keyword(i.ident.to_string());
        visit::visit_item_enum(self, i);
    }

    fn visit_item_trait(&mut self, i: &'ast syn::ItemTrait) {
        self.trait_count += 1;
        self.add_keyword(i.ident.to_string());
        visit::visit_item_trait(self, i);
    }

    fn visit_item_mod(&mut self, i: &'ast syn::ItemMod) {
        self.mod_count += 1;
        // Temporarily skip keyword extraction for i.ident to resolve compilation issues
        visit::visit_item_mod(self, i);
    }

    // Visit identifiers to collect all words
    fn visit_ident(&mut self, i: &'ast syn::Ident) {
        self.add_keyword(i.to_string());
        visit::visit_ident(self, i);
    }

    // Visit literals to collect string/numeric values
    fn visit_lit(&mut self, i: &'ast syn::Lit) {
        self.add_keyword(i.to_token_stream().to_string()); // Corrected: use to_token_stream().to_string()
        visit::visit_lit(self, i);
    }
}

pub fn handle_analyze_duplicates_command(args: AnalyzeDuplicatesArgs) -> Result<()> {
    println!("Starting code feature extraction for similarity analysis...");
    println!("Search path: {}", args.search_path.display());

    let mut all_file_features: HashMap<PathBuf, CodeFeatureExtractor> = HashMap::new();

    for entry in WalkDir::new(&args.search_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            println!("Processing file: {}", path.display());
            match fs::read_to_string(path) {
                Ok(code) => {
                    match syn::parse_file(&code) {
                        Ok(ast) => {
                            let mut extractor = CodeFeatureExtractor::new(path.to_path_buf());
                            extractor.visit_file(&ast);
                            all_file_features.insert(path.to_path_buf(), extractor);
                        },
                        Err(e) => {
                            eprintln!("Error parsing file {}: {}", path.display(), e);
                        }
                    }
                },
                Err(e) => {
                    eprintln!("Error reading file {}: {}", path.display(), e);
                }
            }
        }
    }

    println!("\n--- Extracted Features Report ---");
    for (file_path, features) in &all_file_features { // Changed to &all_file_features
        println!("File: {}", file_path.display());
        println!("  Keywords: {:?}", features.keywords);
        println!("  Function Count: {}", features.fn_count);
        println!("  Struct Count: {}", features.struct_count);
        println!("  Enum Count: {}", features.enum_count);
        println!("  Trait Count: {}", features.trait_count);
        println!("  Module Count: {}", features.mod_count);
        println!("-----------------------------------");
    }

    println!("\nCode feature extraction complete. Ready for similarity scoring.");

    println!("\n--- Similarity Report ---\n"); // Added newline for better formatting
    let mut similarities: Vec<(f64, PathBuf, PathBuf)> = Vec::new();

    let file_paths: Vec<PathBuf> = all_file_features.keys().cloned().collect();

    for i in 0..file_paths.len() {
        for j in (i + 1)..file_paths.len() {
            let path1 = &file_paths[i];
            let path2 = &file_paths[j];

            let features1 = all_file_features.get(path1).unwrap();
            let features2 = all_file_features.get(path2).unwrap();

            let similarity = calculate_similarity(features1, features2);
            similarities.push((similarity, path1.clone(), path2.clone()));
        }
    }

    // Sort by similarity (descending)
    similarities.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

    for (score, path1, path2) in similarities {
        if score > 0.0 { // Only report if there's some similarity
            println!("Similarity: {:.4} between {} and {}", score, path1.display(), path2.display()); // Fixed println! formatting
        }
    }
    println!("\nSimilarity analysis complete.");

    Ok(())
}

// Function to calculate similarity between two CodeFeatureExtractor instances
fn calculate_similarity(f1: &CodeFeatureExtractor, f2: &CodeFeatureExtractor) -> f64 {
    // Jaccard Similarity for Keywords
    let intersection = f1.keywords.intersection(&f2.keywords).count() as f64;
    let union = f1.keywords.union(&f2.keywords).count() as f64;
    let jaccard_similarity = if union > 0.0 { intersection / union } else { 0.0 };

    // Simple Euclidean Distance for AST counts (normalized)
    let ast_vec1 = vec![
        f1.fn_count as f64,
        f1.struct_count as f64,
        f1.enum_count as f64,
        f1.trait_count as f64,
        f1.mod_count as f64,
    ];
    let ast_vec2 = vec![
        f2.fn_count as f64,
        f2.struct_count as f64,
        f2.enum_count as f64,
        f2.trait_count as f64,
        f2.mod_count as f64,
    ];

    // Normalize AST vectors (simple max normalization)
    let max_val = ast_vec1.iter().chain(ast_vec2.iter()).cloned().fold(0.0f64, f64::max);
    let normalized_ast_vec1: Vec<f64> = ast_vec1.iter().map(|&x| if max_val > 0.0 { x / max_val } else { 0.0 }).collect();
    let normalized_ast_vec2: Vec<f64> = ast_vec2.iter().map(|&x| if max_val > 0.0 { x / max_val } else { 0.0 }).collect();

    let euclidean_distance: f64 = normalized_ast_vec1.iter().zip(normalized_ast_vec2.iter())
        .map(|(&x1, &x2)| (x1 - x2).powi(2))
        .sum::<f64>()
        .sqrt();

    // Combine similarities (simple weighted average)
    // Jaccard is between 0 and 1. Euclidean distance is between 0 and sqrt(num_dimensions) (sqrt(5) approx 2.23)
    // We want a score where higher is more similar. So, 1 - normalized_euclidean_distance.
    let max_euclidean_distance = (5.0f64).sqrt(); // Max possible distance for normalized 5D vector
    let normalized_euclidean_similarity = 1.0 - (euclidean_distance / max_euclidean_distance);

    // Simple average for now
    (jaccard_similarity + normalized_euclidean_similarity) / 2.0
}
