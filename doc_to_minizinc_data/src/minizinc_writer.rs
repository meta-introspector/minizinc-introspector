use std::{fs, path::PathBuf};
use crate::logger::LogWriter;
use crate::word_data::WordData;

pub fn write_data_declarations(minizinc_data_dir: &PathBuf, all_relations: &Vec<(String, String, f64)>, logger: &mut LogWriter) -> Result<(), Box<dyn std::error::Error>> {
    let data_declarations_path = minizinc_data_dir.join("data_declarations.mzn");
    let mut data_declarations_content = String::new();
    data_declarations_content.push_str(&format!("int: num_relations = {};\n", all_relations.len()));
    data_declarations_content.push_str("array[1..num_relations] of tuple(string, string): relation_pairs = [\n");
    for (idx, (w1, w2, _)) in all_relations.iter().enumerate() {
        data_declarations_content.push_str(&format!("    (\"{}\", \"{}\")", w1, w2));
        if idx < all_relations.len() - 1 {
            data_declarations_content.push_str(",\n");
        }
    }
    data_declarations_content.push_str("\n];\n");

    data_declarations_content.push_str("array[1..num_relations] of float: desired_distances = [\n");
    for (idx, (_, _, dist)) in all_relations.iter().enumerate() {
        data_declarations_content.push_str(&format!("    {}", dist));
        if idx < all_relations.len() - 1 {
            data_declarations_content.push_str(",\n");
        }
    }
    data_declarations_content.push_str("\n];\n");
    fs::write(&data_declarations_path, data_declarations_content)?;
    logger.log(&format!("Generated {}", data_declarations_path.display()));
    Ok(())
}

pub fn write_chunked_embeddings(minizinc_data_dir: &PathBuf, word_data: &WordData, chunk_size: usize, logger: &mut LogWriter) -> Result<(), Box<dyn std::error::Error>> {
    let num_words = word_data.len();
    let num_chunks = (num_words + chunk_size - 1) / chunk_size;

    for i in 0..num_chunks {
        let start_index = i * chunk_size;
        let end_index = (start_index + chunk_size).min(num_words);

        let chunk_words = &word_data.id_to_word[start_index..end_index];
        let chunk_embeddings = &word_data.embeddings[start_index..end_index];

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
