use std::{collections::HashMap, fs,
	  //io::{//self,
					 //BufReader,
    //BufRead},
    path::PathBuf};
use crate::optimized_embeddings::OptimizedEmbeddings;

pub fn parse_optimized_embeddings(path: &PathBuf) -> Result<OptimizedEmbeddings, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;

    let mut num_words = 0;
    let mut word_map = Vec::new();
    let mut embeddings_map = HashMap::new();
    let mut current_embeddings_values: Vec<f64> = Vec::new();

    let mut in_embeddings_block = false;

    for line in content.lines() {
        if line.starts_with("fixed_num_words =") {
            num_words = line.strip_prefix("fixed_num_words =").unwrap().trim_end_matches(";").trim().parse()?;
        } else if line.starts_with("fixed_word_map =") {
            let map_str = line.strip_prefix("fixed_word_map =").unwrap().trim_end_matches("];").trim();
            word_map = map_str.split(", ").map(|s| s.trim_matches('"').to_string()).collect();
        } else if line.starts_with("fixed_embeddings = array2d(") {
            in_embeddings_block = true;
        } else if in_embeddings_block && line.contains("]);") {
            in_embeddings_block = false;
            // Process the collected embedding values
            for (i, word) in word_map.iter().enumerate() {
                let start_idx = i * 8;
                let end_idx = start_idx + 8;
                let embedding_vec = current_embeddings_values[start_idx..end_idx].to_vec();
                embeddings_map.insert(word.clone(), embedding_vec);
            }
            current_embeddings_values.clear(); // Clear for next block if any
        } else if in_embeddings_block {
            let cleaned_line = line.trim().trim_end_matches(",").trim();
            for val_str in cleaned_line.split(", ") {
                if !val_str.is_empty() {
                    current_embeddings_values.push(val_str.parse()?);
                }
            }
        }
    }

    Ok(OptimizedEmbeddings {
        num_words,
        word_map,
        embeddings: embeddings_map,
    })
}
