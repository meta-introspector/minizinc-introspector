use std::{collections::HashMap, fs, path::PathBuf};
use rand::Rng;
use crate::file_processing::collect_files;
use crate::logger::LogWriter;

pub fn process_files_and_collect_words(
    current_dir: &PathBuf,
    extensions: &[&str],
    word_to_id: &mut HashMap<String, usize>,
    id_to_word: &mut Vec<String>,
    embeddings: &mut Vec<Vec<f64>>,
    rng: &mut impl Rng,
    logger: &mut LogWriter,
) -> Result<(), Box<dyn std::error::Error>> {
    let files_to_process = collect_files(current_dir, extensions)?;

    for file_path in files_to_process {
        let content = match fs::read(&file_path) { // Read as bytes
            Ok(bytes) => String::from_utf8_lossy(&bytes).into_owned(), // Convert bytes to string, replacing invalid UTF-8
            Err(e) => {
                let error_msg = format!("Error reading file {}: {}", file_path.display(), e);
                eprintln!("{}", error_msg); // Still print to stderr
                logger.debug_log(&error_msg); // Also log to file
                continue; // Skip to the next file
            }
        };

        for line in content.lines() {
            for word in line.split_whitespace() {
                let cleaned_word = word.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect::<String>();
                if !cleaned_word.is_empty() {
                    if !word_to_id.contains_key(&cleaned_word) {
                        let id = id_to_word.len();
                        word_to_id.insert(cleaned_word.clone(), id);
                        id_to_word.push(cleaned_word.clone());

                        let embedding: Vec<f64> = (0..8).map(|_| rng.gen_range(-1.0..1.0)).collect();
                        embeddings.push(embedding);
                    }
                }
            }
        }
    }
    Ok(())
}
