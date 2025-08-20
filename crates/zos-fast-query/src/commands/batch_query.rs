use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use clap::Parser;
use std::collections::{HashSet, HashMap};



#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct BatchQueryArgs {
    /// Terms to search for (e.g., "nlp", "wordnet")
    #[arg(short, long, action = clap::ArgAction::Append)]
    pub search_terms: Vec<String>,

    /// Path to the hierarchical term index JSON file
    #[arg(short = 'i', long, default_value_os = "/data/data/com.termux/files/home/storage/github/hierarchical_term_index.json")]
    pub hierarchical_index: PathBuf,

    /// Output results as JSON
    #[arg(long)]
    pub json: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub term: String,
    pub files: HashMap<PathBuf, usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchQueryResult {
    pub total_unique_files: usize,
    pub files_by_term: HashMap<String, HashSet<PathBuf>>,
    pub related_projects_dirs: HashMap<PathBuf, usize>,
}

pub fn run_batch_query(args: BatchQueryArgs) -> Result<(), Box<dyn std::error::Error>> {
    println!("ZOS Fast Query Tool - Batch Query Mode");

    println!("Loading hierarchical term index from: {:?}", args.hierarchical_index);
    let cached_data = fs::read_to_string(&args.hierarchical_index)?;
    let hierarchical_term_index: HashMap<String, HashMap<PathBuf, usize>> = serde_json::from_str(&cached_data)?;

    println!("Successfully loaded index with {} terms.", hierarchical_term_index.len());

    let mut files_by_term: HashMap<String, HashSet<PathBuf>> = HashMap::new();
    let mut all_unique_files: HashSet<PathBuf> = HashSet::new();
    let mut related_projects_dirs: HashMap<PathBuf, usize> = HashMap::new();

    for search_term in &args.search_terms {
        let mut current_term_files: HashSet<PathBuf> = HashSet::new();
        if let Some(file_counts) = hierarchical_term_index.get(search_term) {
            for (file_path, _count) in file_counts {
                current_term_files.insert(file_path.clone());
                all_unique_files.insert(file_path.clone());
                if let Some(parent) = file_path.parent() {
                    *related_projects_dirs.entry(parent.to_path_buf()).or_insert(0) += 1;
                }
            }
        }
        files_by_term.insert(search_term.clone(), current_term_files);
    }

    if args.json {
        let result = BatchQueryResult {
            total_unique_files: all_unique_files.len(),
            files_by_term,
            related_projects_dirs,
        };
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("\n--- Batch Query Results ---");
        println!("Total unique files found across all search terms: {}", all_unique_files.len());

        for (term, files) in &files_by_term {
            println!("\nFiles containing '{}' ({} unique files):", term, files.len());
            for file_path in files {
                println!("  - {:?}", file_path);
            }
        }

        println!("\nRelated projects/directories and their file counts:");
        let mut sorted_related_projects: Vec<(&PathBuf, &usize)> = related_projects_dirs.iter().collect();
        sorted_related_projects.sort_by(|a, b| b.1.cmp(a.1));

        for (dir, count) in sorted_related_projects {
            println!("  - {:?} (files: {})", dir, count);
        }
    }

    Ok(())
}
