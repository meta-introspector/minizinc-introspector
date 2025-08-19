use std::{collections::HashMap, fs, io::{self, Write}, path::PathBuf};
use crate::word_data::WordData;
use rand::Rng;
use crate::cli::Args;
use crate::logger::LogWriter;

pub fn generate_data(args: Args, all_relations: Vec<(String, String, f64)>) -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = std::env::current_dir()?;
    let minizinc_data_dir = current_dir.join("minizinc_data");
    fs::create_dir_all(&minizinc_data_dir)?;

    

    use crate::minizinc_writer::{write_data_declarations, write_chunked_embeddings};

    let log_path = minizinc_data_dir.join("doc_to_minizinc_data.log");
    let mut logger = LogWriter::new(&log_path)?;

    logger.log(&format!("Generating data with args: {:?} and {} relations", args, all_relations.len()));

    let mut word_to_id: HashMap<String, usize> = HashMap::new();
    let mut id_to_word: Vec<String> = Vec::new();
    let mut embeddings: Vec<Vec<f64>> = Vec::new();
    let mut rng = rand::thread_rng();

    let extensions = ["md", "rs", "cpp", "h", "hpp"];
    let mut files_to_process: Vec<PathBuf> = collect_files(&current_dir, &extensions)?;

    use crate::file_processing::{collect_files, process_file};

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

    // Chunking and writing to .dzn files
    let num_words = id_to_word.len();
    let chunk_size = args.chunk_size;
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
        dzn_content.push_str("word_map = [
");
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

    // Report extracted words and embeddings to console
    logger.debug_log("\n--- Extracted Words and Embeddings ---");
    for (id, word) in id_to_word.iter().enumerate() {
        logger.debug_log(&format!("Word: {}, Embedding: {:?}", word, embeddings[id]));
    }
    logger.debug_log("--------------------------------------");

    Ok(())
}