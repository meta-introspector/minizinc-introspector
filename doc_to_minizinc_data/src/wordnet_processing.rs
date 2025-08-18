use std::{fs, io::{BufReader, BufRead}, path::PathBuf};

pub fn generate_wordnet_constraints(wordnet_path: &PathBuf) -> Result<Vec<(String, String, f64)>, Box<dyn std::error::Error>> {
    let file = fs::File::open(wordnet_path)?;
    let reader = BufReader::new(file);

    let mut relations: Vec<(String, String, f64)> = Vec::new();
    let mut current_word = String::new();

    for line_result in reader.lines() {
        let line = line_result?;
        if line.starts_with("word:") {
            current_word = line.replace("word:", "").trim().to_string();
        } else if line.starts_with("synonyms:") {
            let syns_str = line.replace("synonyms:", "").trim().to_string();
            for syn in syns_str.split(", ") {
                if !current_word.is_empty() && !syn.is_empty() {
                    relations.push((current_word.clone(), syn.to_string(), 0.01)); // Small distance for synonyms
                }
            }
        } else if line.starts_with("antonyms:") {
            let ants_str = line.replace("antonyms:", "").trim().to_string();
            for ant in ants_str.split(", ") {
                if !current_word.is_empty() && !ant.is_empty() {
                    relations.push((current_word.clone(), ant.to_string(), 5.0)); // Large distance for antonyms
                }
            }
        }
    }
    Ok(relations)
}