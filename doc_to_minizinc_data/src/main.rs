use std::{fs, io::{self, BufReader, BufRead}, path::PathBuf, collections::HashMap};
use walkdir::WalkDir;
use rand::Rng;
use clap::Parser;

const WINDOW_SIZE: usize = 5;

fn generate_wordnet_constraints(wordnet_path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
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

    let mut dzn_content = String::new();
    dzn_content.push_str(&format!("num_relations = {};\n", relations.len()));
    dzn_content.push_str("relation_pairs = [|");
    let pairs_str = relations.iter()
        .map(|(w1, w2, _)| format!("\"{}\" \"{}\"", w1, w2))
        .collect::<Vec<String>>()
        .join("|, |");
    dzn_content.push_str(&pairs_str);
    dzn_content.push_str("|];\n");

    dzn_content.push_str("desired_distances = [");
    let distances_str = relations.iter()
        .map(|(_, _, dist)| dist.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    dzn_content.push_str(&distances_str);
    dzn_content.push_str("];\n");

    Ok(dzn_content)
}


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Size of each chunk for word embeddings
    #[arg(short, long, default_value_t = 100)]
    chunk_size: usize,
}

fn format_pair(w1:&String, w2:&String) -> String {
    return format!("\"{}\" \"{}\"", w1, w2)
}

fn format_triple(w1:&String, w2:&String, w3:&String) -> String {
    return format!("\"{}\" \"{}\" \"{}\"", w1, w2, w3)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let chunk_size = args.chunk_size;

    let docs_categorized_dir = PathBuf::from("/data/data/com.termux/files/home/storage/github/libminizinc/docs/categorized");
    let simulated_wordnet_path = PathBuf::from("/data/data/com.termux/files/home/storage/github/libminizinc/minizinc_data/simulated_wordnet.txt");
    let logical_relationships_dzn_path = PathBuf::from("/data/data/com.termux/files/home/storage/github/libminizinc/minizinc_data/logical_relationships.dzn");

    // Generate logical relationships from simulated WordNet
    let wordnet_dzn_content = generate_wordnet_constraints(&simulated_wordnet_path)?;
    fs::write(&logical_relationships_dzn_path, wordnet_dzn_content)?;
    println!("Generated logical relationships from WordNet: {:?}", logical_relationships_dzn_path);

    let mut word_to_embedding: HashMap<String, Vec<f64>> = HashMap::new();
    let mut bigram_counts: HashMap<(String, String), usize> = HashMap::new();
    let mut trigram_counts: HashMap<(String, String, String), usize> = HashMap::new();
    let mut rng = rand::thread_rng();

    for entry in WalkDir::new(&docs_categorized_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| { let ext_str = ext.to_str().unwrap_or(""); ext_str == "md" || ext_str == "rs" || ext_str == "cpp" || ext_str == "h" || ext_str == "hpp" }) {
            let content = fs::read_to_string(&path)?;
            
            // Simple word extraction: lowercase, filter non-alphabetic, split by whitespace
            let cleaned_words: Vec<String> = content.to_lowercase().split_whitespace()
                .filter_map(|word| {
                    let cleaned_word: String = word.chars()
                        .filter(|c| c.is_alphabetic())
                        .collect();
                    if cleaned_word.is_empty() { None } else { Some(cleaned_word) }
                })
                .collect();

            for i in 0..cleaned_words.len() {
                let word1 = &cleaned_words[i];
                if !word_to_embedding.contains_key(word1) {
                    let embedding: Vec<f64> = (0..8).map(|_| rng.gen_range(-1.0..1.0)).collect();
                    word_to_embedding.insert(word1.clone(), embedding);
                }

                // Bigram counting
                for j in (i + 1)..std::cmp::min(i + WINDOW_SIZE, cleaned_words.len()) {
                    let word2 = &cleaned_words[j];
                    *bigram_counts.entry((word1.clone(), word2.clone())).or_insert(0) += 1;
                }

                // Trigram counting
                for j in (i + 1)..std::cmp::min(i + WINDOW_SIZE, cleaned_words.len()) {
                    for k in (j + 1)..std::cmp::min(i + WINDOW_SIZE, cleaned_words.len()) {
                        let word2 = &cleaned_words[j];
                        let word3 = &cleaned_words[k];
                        *trigram_counts.entry((word1.clone(), word2.clone(), word3.clone())).or_insert(0) += 1;
                    }
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

    // Output co-occurrence data
    let co_occurrence_dzn_path = PathBuf::from("/data/data/com.termux/files/home/storage/github/libminizinc/minizinc_data/co_occurrence_data.dzn");
    let mut co_occurrence_dzn_content = String::new();

    // Bigrams
    co_occurrence_dzn_content.push_str(&format!("num_bigrams = {};\n", bigram_counts.len()));
    co_occurrence_dzn_content.push_str("bigram_pairs = [|");
    let bigram_pairs_str = bigram_counts.keys()
        .map(|(w1, w2)| format_pair( w1, w2))
        .collect::<Vec<String>>()
        .join("|, |");
    co_occurrence_dzn_content.push_str(&bigram_pairs_str);
    co_occurrence_dzn_content.push_str("|];\n");

    co_occurrence_dzn_content.push_str("bigram_counts = [");
    let bigram_counts_str = bigram_counts.values()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    co_occurrence_dzn_content.push_str(&bigram_counts_str);
    co_occurrence_dzn_content.push_str("];\n");

    // Trigrams
    co_occurrence_dzn_content.push_str(&format!("num_trigrams = {};\n", trigram_counts.len()));
    co_occurrence_dzn_content.push_str("trigram_triples = [|");
    let trigram_triples_str = trigram_counts.keys()
        .map(|(w1, w2, w3)| format_triple( w1, w2, w3))
        .collect::<Vec<String>>()
        .join("|, |");
    co_occurrence_dzn_content.push_str(&trigram_triples_str);
    co_occurrence_dzn_content.push_str("|];\n");

    co_occurrence_dzn_content.push_str("trigram_counts = [");
    let trigram_counts_str = trigram_counts.values()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    co_occurrence_dzn_content.push_str(&trigram_counts_str);
    co_occurrence_dzn_content.push_str("];\n");

    fs::write(&co_occurrence_dzn_path, co_occurrence_dzn_content)?;
    println!("Generated co-occurrence data: {:?}", co_occurrence_dzn_path);

    Ok(())
}
