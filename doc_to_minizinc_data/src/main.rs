use std::{fs, path::PathBuf, collections::HashMap};
use walkdir::WalkDir;
use rand::Rng;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Size of each chunk for word embeddings
    #[arg(short, long, default_value_t = 100)]
    chunk_size: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let chunk_size = args.chunk_size;

    let docs_categorized_dir = PathBuf::from("/data/data/com.termux/files/home/storage/github/libminizinc/docs/categorized");

    let mut word_to_embedding: HashMap<String, Vec<f64>> = HashMap::new();
    let mut rng = rand::thread_rng();

    for entry in WalkDir::new(&docs_categorized_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| { let ext_str = ext.to_str().unwrap_or(""); ext_str == "md" || ext_str == "rs" || ext_str == "cpp" || ext_str == "h" || ext_str == "hpp" }) {
            let content = fs::read_to_string(&path)?;
            
            // Simple word extraction: lowercase, filter non-alphabetic, split by whitespace
            for word in content.to_lowercase().split_whitespace() {
                let cleaned_word: String = word.chars()
                    .filter(|c| c.is_alphabetic())
                    .collect();
                
                if !cleaned_word.is_empty() && !word_to_embedding.contains_key(&cleaned_word) {
                    // Assign a random 8-dimensional vector (simulated embedding)
                    let embedding: Vec<f64> = (0..8).map(|_| rng.gen_range(-1.0..1.0)).collect();
                    word_to_embedding.insert(cleaned_word, embedding);
                }
            }
        }
    }

    // Sort words alphabetically to ensure consistent indexing
    let mut sorted_words: Vec<String> = word_to_embedding.keys().cloned().collect();
    sorted_words.sort();

    // Iterate through sorted_words in chunks
    for (chunk_index, chunk) in sorted_words.chunks(chunk_size).enumerate() {
        let output_dzn_path = PathBuf::from(format!("/data/data/com.termux/files/home/storage/github/libminizinc/minizinc_data/word_embeddings_chunks/word_embeddings_chunk_{}.dzn", chunk_index));

        let mut dzn_content = String::new();

        // Output num_words for the current chunk
        dzn_content.push_str(&format!("num_words = {};\n\n", chunk.len()));

        // Output word_map for the current chunk
        dzn_content.push_str("word_map = [\"");
        dzn_content.push_str(&chunk.join("\", \""));
        dzn_content.push_str("\"];\n\n");

        // Output embeddings using array2d constructor for the current chunk
        dzn_content.push_str(&format!("embeddings = array2d(1..{}, 1..8, [\n", chunk.len()));
        let mut all_embedding_values: Vec<String> = Vec::new();
        for word in chunk {
            if let Some(embedding) = word_to_embedding.get(word) {
                all_embedding_values.extend(embedding.iter().map(|f| f.to_string()));
            }
        }
        dzn_content.push_str(&all_embedding_values.join(", "));
        dzn_content.push_str("\n]);\n");

        fs::write(&output_dzn_path, dzn_content)?;
        println!("Generated MiniZinc data file for chunk {}: {:?}", chunk_index, output_dzn_path);

        // Report for the current chunk
        println!("\n--- Word Embeddings Report for Chunk {} ---", chunk_index);
        for word in chunk {
            if let Some(embedding) = word_to_embedding.get(word) {
                let embedding_str = embedding.iter().map(|f| format!("{:.4}", f)).collect::<Vec<String>>().join(", ");
                println!("Word: {:<20} Embedding: {}", word, embedding_str); // Corrected line
            }
        }
        println!("------------------------------------------");
    }

    Ok(())
}