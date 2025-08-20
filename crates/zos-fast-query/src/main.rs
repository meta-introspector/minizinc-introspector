use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use clap::Parser;
use std::collections::{HashSet, HashMap};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the JSON file containing shared terms
    #[arg(short, long)]
    input_file: PathBuf,

    /// Term to filter by (e.g., "rust_file_finder")
    #[arg(short, long)]
    filter_term: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SharedTermsGroup {
    group_id: usize,
    files: Vec<PathBuf>,
    terms: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("ZOS Fast Query Tool - Reading Shared Terms");

    println!("Loading shared terms from: {:?}", args.input_file);
    let cached_data = fs::read_to_string(&args.input_file)?;
    let shared_terms_groups: Vec<SharedTermsGroup> = serde_json::from_str(&cached_data)?;

    println!("Successfully loaded {} shared term groups.", shared_terms_groups.len());

    let mut files_from_other_projects: HashSet<PathBuf> = HashSet::new();
    let mut related_projects_dirs: HashMap<PathBuf, usize> = HashMap::new();

    for group in shared_terms_groups {
        let mut is_relevant_group = false;
        for file_path in &group.files {
            let file_path_str = file_path.to_string_lossy();
            let matches_filter_term = if let Some(ref term) = args.filter_term {
                file_path_str.contains(term)
            } else {
                false // If no filter term is provided, it doesn't match by term
            };

            if matches_filter_term || *file_path == args.input_file {
                is_relevant_group = true;
                break;
            }
        }

        if is_relevant_group {
            // This group is relevant because it contains files matching the filter term or the input file
            for file_path in group.files {
                let file_path_str = file_path.to_string_lossy();
                let matches_filter_term = if let Some(ref term) = args.filter_term {
                    file_path_str.contains(term)
                } else {
                    false // If no filter term is provided, it doesn't match by term
                };

                // Collect files from *other* projects within this relevant group
                if !matches_filter_term && *file_path != args.input_file {
                    if files_from_other_projects.insert(file_path.clone()) {
                        // If the file was newly inserted (i.e., it's unique and from an 'other' project)
                        if let Some(parent) = file_path.parent() {
                            *related_projects_dirs.entry(parent.to_path_buf()).or_insert(0) += 1;
                        }
                    }
                }
            }
        }
    }

    println!("\n--- Analysis Results ---");
    println!("Total unique files from other projects/directories related to the filter term or the input file: {}", files_from_other_projects.len());

    println!("\nRelated projects/directories and their file counts (based on shared terms):");
    let mut sorted_related_projects: Vec<(&PathBuf, &usize)> = related_projects_dirs.iter().collect();
    sorted_related_projects.sort_by(|a, b| b.1.cmp(a.1));

    for (dir, count) in sorted_related_projects {
        println!("  - {:?} (files: {})", dir, count);
    }

    Ok(())
}