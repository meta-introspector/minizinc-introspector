use std::{fs, path::PathBuf, collections::HashMap};
use walkdir::WalkDir;
use rand::Rng;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let docs_categorized_dir = PathBuf::from("/data/data/com.termux/files/home/storage/github/libminizinc/docs/categorized");
    let output_dzn_path = PathBuf::from("/data/data/com.termux/files/home/storage/github/libminizinc/word_embeddings.dzn");

    let mut word_to_embedding: HashMap<String, Vec<f64>> = HashMap::new();
    let mut rng = rand::thread_rng();

    for entry in WalkDir::new(&docs_categorized_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
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

    // Generate .dzn file
    let mut dzn_content = String::new();

    // Output num_words
    dzn_content.push_str("num_words = ");
    dzn_content.push_str(&sorted_words.len().to_string());
    dzn_content.push_str(";\n\n");

    // Output word_map (array of strings)
    dzn_content.push_str("word_map = [\"");
    dzn_content.push_str(&sorted_words.join("\", \""));
    dzn_content.push_str("\"];\n\n");

    // Output embeddings using array2d constructor
    dzn_content.push_str("embeddings = array2d(1..");
    dzn_content.push_str(&sorted_words.len().to_string());
    dzn_content.push_str(", 1..8, [\n");
    let mut all_embedding_values: Vec<String> = Vec::new();
    for word in &sorted_words {
        if let Some(embedding) = word_to_embedding.get(word) {
            all_embedding_values.extend(embedding.iter().map(|f| f.to_string()));
        }
    }
    dzn_content.push_str(&all_embedding_values.join(", ")); // Join all floats with comma and space
    dzn_content.push_str("\n]);\n");

    fs::write(&output_dzn_path, dzn_content)?;
    println!("Generated MiniZinc data file: {:?}", output_dzn_path);

    Ok(())
}
