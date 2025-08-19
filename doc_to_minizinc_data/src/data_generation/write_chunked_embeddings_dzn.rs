use std::{fs, path::PathBuf};
use crate::logger::LogWriter;

pub fn write_chunked_embeddings_dzn(
    id_to_word: &Vec<String>,
    embeddings: &Vec<Vec<f64>>,
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

        fs::write(&dzn_path, dzn_content)?;
        logger.log(&format!("Generated {}", dzn_path.display()));
    }
    Ok(())
}
