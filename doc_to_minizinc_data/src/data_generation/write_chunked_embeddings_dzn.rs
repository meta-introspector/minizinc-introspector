use std::{fs, path::PathBuf, collections::HashMap}; // Import HashMap
use anyhow::Result;
//use crate::prelude::LogWriter;
use crate::logger::LogWriter;
pub fn write_chunked_embeddings_dzn(
    id_to_word: &HashMap<u32, String>,
    word_to_id: &std::collections::HashMap<String, usize>,
    embeddings: &HashMap<u32, Vec<f64>>,
    all_relations: &Vec<(String, String, f64)>,
    chunk_size: usize,
    minizinc_data_dir: &PathBuf,
    _fixed_id_to_word: &HashMap<u32, String>,
    fixed_id_to_embedding: &HashMap<u32, Vec<f64>>,
    logger: &mut LogWriter,
) -> Result<()> {
    let num_words = id_to_word.len();
    let num_chunks = num_words.div_ceil(chunk_size);

    // Collect all words and embeddings into sorted Vecs for chunking
    let mut sorted_ids: Vec<u32> = id_to_word.keys().cloned().collect();
    sorted_ids.sort_unstable();

    for i in 0..num_chunks {
        let start_index = i * chunk_size;
        let end_index = (start_index + chunk_size).min(num_words);

        let chunk_ids = &sorted_ids[start_index..end_index];

        let dzn_filename = format!("word_embeddings_chunk_{i}.dzn");
        let dzn_path = minizinc_data_dir.join(dzn_filename);

        let mut dzn_content = String::new();
        dzn_content.push_str(&format!("num_words = {};\n", chunk_ids.len()));
        dzn_content.push_str("word_map = [\n");
        for (idx, &id) in chunk_ids.iter().enumerate() {
            let word = id_to_word.get(&id).unwrap();
            dzn_content.push_str(&format!("\"{word}\""));
            if idx < chunk_ids.len() - 1 {
                dzn_content.push_str(", ");
            }
        }
        dzn_content.push_str("\n];\n");

        dzn_content.push_str(&format!("embeddings = array2d(1..{}, 1..8, [\n", chunk_ids.len()));
        for (idx, &id) in chunk_ids.iter().enumerate() {
            let embedding = embeddings.get(&id).unwrap();
            dzn_content.push_str(&format!("  {}", embedding.iter().map(|f| f.to_string()).collect::<Vec<String>>().join(", ")));
            if idx < chunk_ids.len() - 1 {
                dzn_content.push_str(",\n");
            }
        }
        dzn_content.push_str("\n]);\n");

        // Fixed embeddings logic
        let mut fixed_word_indices_in_chunk = Vec::new();
        let mut fixed_word_embeddings_values = Vec::new();

        for (local_idx, &global_id) in chunk_ids.iter().enumerate() {
            if let Some(fixed_embedding_vec) = fixed_id_to_embedding.get(&global_id) {
                // This word is in the current chunk AND is a fixed word
                fixed_word_indices_in_chunk.push((local_idx + 1) as u32); // MiniZinc indices are 1-based
                fixed_word_embeddings_values.extend_from_slice(fixed_embedding_vec);
            }
        }

        dzn_content.push_str(&format!("num_fixed_words = {};\n", fixed_word_indices_in_chunk.len()));
        dzn_content.push_str(&format!("fixed_word_indices = [{}];\n", fixed_word_indices_in_chunk.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(", ")));
        dzn_content.push_str(&format!("fixed_word_embeddings = array2d(1..{}, 1..8, [{}]);\n", fixed_word_indices_in_chunk.len(), fixed_word_embeddings_values.iter().map(|f| f.to_string()).collect::<Vec<String>>().join(", ")));

        // Filter relations relevant to this chunk and map words to chunk-local IDs
        let mut chunk_relation_pairs = Vec::new();
        let mut chunk_desired_distances = Vec::new();

        // Create a map from global ID to chunk-local ID for this chunk
        let global_to_local_id: HashMap<u32, u32> = chunk_ids.iter().enumerate().map(|(local_idx, &global_id)| (global_id, local_idx as u32 + 1)).collect();

        for (word1_str, word2_str, desired_dist) in all_relations.iter() {
            if let (Some(id1_global), Some(id2_global)) = (word_to_id.get(word1_str).map(|&x| x as u32), word_to_id.get(word2_str).map(|&x| x as u32)) {
                if let (Some(id1_local), Some(id2_local)) = (global_to_local_id.get(&id1_global), global_to_local_id.get(&id2_global)) {
                    chunk_relation_pairs.push(format!("({id1_local},{id2_local}"));
                    chunk_desired_distances.push(desired_dist.to_string());
                }
            }
        }

        dzn_content.push_str(&format!("num_relations = {};\n", chunk_relation_pairs.len()));
        dzn_content.push_str(&format!("relation_pairs = [{}];\n", chunk_relation_pairs.join(", ")));
        dzn_content.push_str(&format!("desired_distances = [{}];\n", chunk_desired_distances.join(", ")));

        fs::write(&dzn_path, dzn_content)?;
        logger.log(&format!("Generated {}", dzn_path.display()));
    }
    Ok(())
}
