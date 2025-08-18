use std::{fs, io, path::PathBuf};
use rand::Rng;
use crate::word_data::WordData;
use crate::logger::LogWriter;

pub fn collect_files(dir: &PathBuf, extensions: &[&str]) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                files.extend(collect_files(&path, extensions)?);
            } else if path.is_file() {
                if let Some(ext) = path.extension() {
                    if extensions.contains(&ext.to_str().unwrap_or("")) {
                        files.push(path);
                    }
                }
            }
        }
    }
    Ok(files)
}

pub fn process_file<T: Rng>(file_path: &PathBuf, word_data: &mut WordData, rng: &mut T, logger: &mut LogWriter) -> Result<(), Box<dyn std::error::Error>> {
    let content = match fs::read(&file_path) { // Read as bytes
        Ok(bytes) => String::from_utf8_lossy(&bytes).into_owned(), // Convert bytes to string, replacing invalid UTF-8
        Err(e) => {
            let error_msg = format!("Error reading file {}: {}", file_path.display(), e);
            eprintln!("{}", error_msg); // Still print to stderr
            logger.debug_log(&error_msg); // Also log to file
            return Ok(()); // Skip to the next file
        }
    };

    for line in content.lines() {
        for word in line.split_whitespace() {
            let cleaned_word = word.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect::<String>();
            if !cleaned_word.is_empty() {
                if !word_data.contains_word(&cleaned_word) {
                    let embedding: Vec<f64> = (0..8).map(|_| rng.gen_range(-1.0..1.0)).collect();
                    word_data.add_word(cleaned_word, embedding);
                }
            }
        }
    }
    Ok(())
}
