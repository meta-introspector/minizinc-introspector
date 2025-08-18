use clap::Parser;
use std::{collections::HashMap, fs, path::PathBuf};
use walkdir::WalkDir;

mod optimized_embeddings;
mod parse_optimized_embeddings;
mod logical_relationships;

use optimized_embeddings::OptimizedEmbeddings;
use parse_optimized_embeddings::parse_optimized_embeddings;
use logical_relationships::{
    //LogicalRelationships,
    parse_logical_relationships};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the logical relationships DZN file
    #[arg(long)]
    logical_relationships_dzn: PathBuf,
    /// Directory containing the MiniZinc log files with optimized embeddings
    #[arg(long)]
    minizinc_logs_dir: PathBuf,
}

fn euclidean_distance(vec1: &[f64], vec2: &[f64]) -> f64 {
    vec1.iter().zip(vec2.iter()).map(|(&x1, &x2)| (x1 - x2).powi(2)).sum::<f64>().sqrt()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("Loading logical relationships from: {:?}", args.logical_relationships_dzn);
    let logical_relationships = parse_logical_relationships(&args.logical_relationships_dzn)?;
    println!("Loaded logical relationships: {:?}", logical_relationships);

    println!("Loading optimized embeddings from logs in: {:?}", args.minizinc_logs_dir);
    let mut all_optimized_embeddings: HashMap<String, OptimizedEmbeddings> = HashMap::new();
    let mut loss_report: HashMap<String, f64> = HashMap::new();

    for entry in WalkDir::new(&args.minizinc_logs_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "log") {
            println!("  Parsing log file: {:?}", path);
            let path_buf = path.to_path_buf();
            let optimized_embeddings = parse_optimized_embeddings(&path_buf)?;
            all_optimized_embeddings.insert(path.file_stem().unwrap().to_string_lossy().to_string(), optimized_embeddings);

            // Extract loss from the log file
            let log_content = fs::read_to_string(&path_buf)?;
            if let Some(loss_line) = log_content.lines().find(|l| l.starts_with("Loss = ")) {
                let loss_str = loss_line.strip_prefix("Loss = ").unwrap().trim();
                if let Ok(loss_val) = loss_str.parse::<f64>() {
                    loss_report.insert(path.file_stem().unwrap().to_string_lossy().to_string(), loss_val);
                }
            }
        }
    }

    println!("Loaded {} optimized embedding chunks.", all_optimized_embeddings.len());

    // Loss Report
    println!("\n--- Loss Report ---");
    for (chunk_name, loss_val) in &loss_report {
        println!("  Chunk {}: Loss = {:.4}", chunk_name, loss_val);
    }
    println!("-------------------");

    // Query words and check constraints
    println!("\n--- Constraint Check Report ---");
    for (chunk_name, embeddings_data) in &all_optimized_embeddings {
        println!("\nChunk: {}", chunk_name);
        for (i, (word1, word2)) in logical_relationships.relation_pairs.iter().enumerate() {
            if let (Some(emb1), Some(emb2)) = (embeddings_data.embeddings.get(word1), embeddings_data.embeddings.get(word2)) {
                let actual_dist = euclidean_distance(emb1, emb2);
                let desired_dist = logical_relationships.desired_distances[i];
                println!("  Distance({}, {}) - Actual: {:.4}, Desired: {:.4}, Diff: {:.4}",
                           word1, word2, actual_dist, desired_dist, (actual_dist - desired_dist).abs());
            } else {
                println!("  Warning: One or both words ({}, {}) not found in chunk {}.", word1, word2, chunk_name);
            }
        }
    }
    println!("-------------------------------");

    // Antonyms that are close & Possible Antonyms
    println!("\n--- Antonym Analysis Report ---");
    let antonym_threshold_close = 1.0; // If actual distance is below this for an antonym
    let possible_antonym_threshold = 4.0; // If actual distance is above this for non-antonyms

    for (chunk_name, embeddings_data) in &all_optimized_embeddings {
        println!("\nChunk: {}", chunk_name);
        let mut antonyms_too_close = Vec::new();
        let mut suggested_antonyms = Vec::new();

        // Check explicit antonyms that are too close
        for (i, (word1, word2)) in logical_relationships.relation_pairs.iter().enumerate() {
            let desired_dist = logical_relationships.desired_distances[i];
            // Assuming antonyms have a desired_distance > 1.0 (from simulated_wordnet.txt)
            if desired_dist > 1.0 {
                if let (Some(emb1), Some(emb2)) = (embeddings_data.embeddings.get(word1), embeddings_data.embeddings.get(word2)) {
                    let actual_dist = euclidean_distance(emb1, emb2);
                    if actual_dist < antonym_threshold_close {
                        antonyms_too_close.push(format!("({}, {}) - Actual: {:.4}, Desired: {:.4}", word1, word2, actual_dist, desired_dist));
                    }
                }
            }
        }

        // Look for possible antonyms (pairs with large distance not explicitly defined as antonyms)
        // This can be computationally expensive, so we'll limit it to words present in the current chunk
        let words_in_chunk: Vec<&String> = embeddings_data.embeddings.keys().collect();
        for i in 0..words_in_chunk.len() {
            for j in (i + 1)..words_in_chunk.len() {
                let word1 = words_in_chunk[i];
                let word2 = words_in_chunk[j];

                // Skip if already an explicit relationship
                let is_explicit_relation = logical_relationships.relation_pairs.iter().any(|(w1, w2_rel)| {
                    (w1 == word1 && w2_rel == word2) || (w1 == word2 && w2_rel == word1)
                });

                if !is_explicit_relation {
                    if let (Some(emb1), Some(emb2)) = (embeddings_data.embeddings.get(word1), embeddings_data.embeddings.get(word2)) {
                        let actual_dist = euclidean_distance(emb1, emb2);
                        if actual_dist > possible_antonym_threshold {
                            suggested_antonyms.push(format!("({}, {}) - Distance: {:.4}", word1, word2, actual_dist));
                        }
                    }
                }
            }
        }

        if !antonyms_too_close.is_empty() {
            println!("  Antonyms that are too close:");
            for ant in antonyms_too_close {
                println!("    - {}", ant);
            }
        } else {
            println!("  No explicit antonyms found to be too close.");
        }

        if !suggested_antonyms.is_empty() {
            println!("  Suggested possible antonyms:");
            for ant in suggested_antonyms {
                println!("    - {}", ant);
            }
        } else {
            println!("  No new possible antonyms suggested.");
        }
    }
    println!("-------------------------------");

    Ok(())
}

