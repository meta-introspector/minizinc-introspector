use std::{collections::HashMap, fs, path::PathBuf};
use crate::logger::LogWriter;

pub fn write_data_declarations_mzn(
    all_relations: &Vec<(String, String, f64)>,
    word_to_id: &HashMap<String, usize>,
    id_to_word: &Vec<String>,
    embeddings: &Vec<Vec<f64>>,
    minizinc_data_dir: &PathBuf,
    logger: &mut LogWriter,
) -> Result<(), Box<dyn std::error::Error>> {
    let data_declarations_path = minizinc_data_dir.join("data_declarations.mzn");
    let mut data_declarations_content = String::new();

    // Filter relations and write num_relations, relation_pairs, and desired_distances
    let mut filtered_relations = Vec::new();
    for (word1, word2, distance) in all_relations.iter() {
        if word_to_id.contains_key(word1.as_str()) && word_to_id.contains_key(word2.as_str()) {
            filtered_relations.push((word1, word2, distance));
        }
    }
    data_declarations_content.push_str(&format!("num_relations = {};\n", filtered_relations.len()));
    data_declarations_content.push_str("relation_pairs = [\n");
    for (idx, (word1, word2, _)) in filtered_relations.iter().enumerate() {
        let id1 = word_to_id.get(word1.as_str()).unwrap();
        let id2 = word_to_id.get(word2.as_str()).unwrap();
        data_declarations_content.push_str(&format!("  ({}, {})", id1, id2));
        if idx < filtered_relations.len() - 1 {
            data_declarations_content.push_str(",\n");
        }
    }
    data_declarations_content.push_str("\n];\n");

    data_declarations_content.push_str("desired_distances = [\n");
    for (idx, (_, _, distance)) in filtered_relations.iter().enumerate() {
        data_declarations_content.push_str(&format!("  {}", distance));
        if idx < filtered_relations.len() - 1 {
            data_declarations_content.push_str(",\n");
        }
    }
    data_declarations_content.push_str("\n];\n");

    fs::write(&data_declarations_path, data_declarations_content)?;
    logger.log(&format!("Generated {}", data_declarations_path.display()));
    Ok(())
}

