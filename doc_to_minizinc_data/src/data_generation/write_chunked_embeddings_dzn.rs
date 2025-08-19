use std::{fs, path::PathBuf};
use crate::logger::LogWriter;

pub fn write_chunked_embeddings_dzn(
    id_to_word: &Vec<String>,
    word_to_id: &std::collections::HashMap<String, usize>,
    embeddings: &Vec<Vec<f64>>,
    all_relations: &Vec<(String, String, f64)>,
    chunk_size: usize,
    minizinc_data_dir: &PathBuf,
    logger: &mut LogWriter,
) -> Result<(), Box<dyn std::error::Error>> {
    let num_words = id_to_word.len();
    let num_chunks = (num_words + chunk_size - 1) / chunk_size;

    for i in 0..num_chunks {
        let start_index = i * chunk_size;
        let end_index = (start_index + chunk_size).min(num_words);

        let chunk_words = &id_to_word[start_index..end_index];
        let chunk_embeddings = &embeddings[start_index..end_index];

        let dzn_filename = format!("word_embeddings_chunk_{}.dzn", i);
        let dzn_path = minizinc_data_dir.join(dzn_filename);

        let mut dzn_content = String::new();
        dzn_content.push_str(&format!("num_words = {};\n", chunk_words.len()));
        dzn_content.push_str("word_map = [\n");
        for (idx, word) in chunk_words.iter().enumerate() {
            dzn_content.push_str(&format!("\"{}\"", word));
            if idx < chunk_words.len() - 1 {
                dzn_content.push_str(", ");
            }
        }
        dzn_content.push_str("\n];\n");

        dzn_content.push_str(&format!("embeddings = array2d(1..{}, 1..8, [\n", chunk_words.len()));
        for (idx, embedding) in chunk_embeddings.iter().enumerate() {
            dzn_content.push_str(&format!("  {}", embedding.iter().map(|f| f.to_string()).collect::<Vec<String>>().join(", ")));
            if idx < chunk_embeddings.len() - 1 {
                dzn_content.push_str(",\n");
            }
        }
        dzn_content.push_str("\n]);\n");

        // Filter relations relevant to this chunk and map words to chunk-local IDs
        let mut chunk_relation_pairs = Vec::new();
        let mut chunk_desired_distances = Vec::new();

        for (word1_str, word2_str, desired_dist) in all_relations.iter() {
            if let (Some(&id1_global), Some(&id2_global)) = (word_to_id.get(word1_str), word_to_id.get(word2_str)) {
                // Check if both words are within the current chunk's global index range
                if id1_global >= start_index && id1_global < end_index &&
                   id2_global >= start_index && id2_global < end_index {
                    // Calculate chunk-local IDs (1-based for MiniZinc)
                    let id1_local = id1_global - start_index + 1;
                    let id2_local = id2_global - start_index + 1;
                    chunk_relation_pairs.push(format!("({},{}", id1_local, id2_local));
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
