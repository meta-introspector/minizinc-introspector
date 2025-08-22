## Change Request: Novelty Tracking and Chunk Linking for Poem Analysis

**Date:** 2025-08-22

**Author:** Gemini CLI Agent (Rust Edit Manager)

**Purpose:**
To propose the implementation of a system for tracking word novelty across poem chunks and establishing links between chunks based on shared vocabulary. This forms the initial layer of a syntax-aware text model.

**Background:**
Building upon the "bag of words" concept, the goal is to move towards a more nuanced understanding of text by identifying new terms and mapping relationships between textual units. This CRQ outlines the first phase of this development, focusing on core logic without full NLP integration.

**Proposed Change:**

### 1. Core Data Structures

We will introduce the following Rust data structures to manage chunks, vocabulary, and links:

*   **`TextChunk`**: Represents a processed sentence from a poem.
*   **`NoveltyTracker`**: Manages the overall state, including the global vocabulary, chunk storage, and linking logic.

### 2. Initial Vocabulary

The words "the" and "a" will be pre-loaded into the `global_vocabulary` as initially known terms.

### 3. Chunking Strategy

Poems will be split into **sentences** for processing. Each sentence will form a `TextChunk`.

### 4. Novelty Tracking and Linking Logic

The `NoveltyTracker` will implement the following logic:
*   When a sentence (chunk) is processed, its words are extracted (lowercased, alphanumeric, >1 character).
*   If a word is encountered for the first time (i.e., not in `global_vocabulary`), it is marked as "novel," added to the `global_vocabulary`, and its first appearance (the current chunk ID) is recorded.
*   If a word is already in `global_vocabulary` but was previously marked as novel in *another* chunk, a link is established between the current chunk and the chunk where that word first appeared as novel.
*   Additionally, links will be established between the current chunk and any other chunks that contain the same word, creating a network of shared vocabulary.

### Proposed Rust Code:

```rust
use std::{fs, path::Path, collections::{HashMap, HashSet}};
use anyhow::{Result, anyhow};
use regex::Regex;

// Represents a processed chunk of text
#[derive(Debug, Clone)]
struct TextChunk {
    id: usize, // Unique ID for the chunk
    source_file: String, // Path to the poem file
    content: String, // The actual text of the chunk (e.g., a sentence)
    words: HashSet<String>, // Unique words in this chunk (lowercased)
    // Placeholder for future syntax fields (POS, lemma, dependencies)
}

// Manages the overall vocabulary and chunk relationships
struct NoveltyTracker {
    next_chunk_id: usize,
    global_vocabulary: HashSet<String>, // All unique words encountered so far
    novel_word_first_appearance: HashMap<String, usize>, // Word -> ID of chunk where it first appeared as novel
    chunks: HashMap<usize, TextChunk>, // ID -> TextChunk
    word_to_chunks: HashMap<String, HashSet<usize>>, // Word -> IDs of all chunks containing it
    chunk_links: HashMap<usize, HashSet<usize>>, // Chunk ID -> IDs of linked chunks (based on shared words)
}

impl NoveltyTracker {
    fn new() -> Self {
        let mut initial_vocab = HashSet::new();
        initial_vocab.insert("the".to_string());
        initial_vocab.insert("a".to_string());

        NoveltyTracker {
            next_chunk_id: 0,
            global_vocabulary: initial_vocab,
            novel_word_first_appearance: HashMap::new(),
            chunks: HashMap::new(),
            word_to_chunks: HashMap::new(),
            chunk_links: HashMap::new(),
        }
    }

    /// Processes a sentence, identifies novel words, and updates the tracker.
    fn process_sentence(&mut self, sentence: &str, source_file: &Path) -> Result<()> {
        let current_chunk_id = self.next_chunk_id;
        self.next_chunk_id += 1;

        let word_regex = Regex::new(r"\b\w+\b").unwrap();
        let mut chunk_words = HashSet::new();
        let mut linked_chunks_for_this = HashSet::new();

        for mat in word_regex.find_iter(sentence) {
            let word = mat.as_str().to_lowercase();
            if word.len() > 1 { // Basic filtering: ignore single-character words
                chunk_words.insert(word.clone());

                if !self.global_vocabulary.contains(&word) {
                    // This is a truly novel word
                    self.global_vocabulary.insert(word.clone());
                    self.novel_word_first_appearance.insert(word.clone(), current_chunk_id);
                } else {
                    // Word is known. Check if it was novel in another chunk and link.
                    if let Some(&first_novel_chunk_id) = self.novel_word_first_appearance.get(&word) {
                        if first_novel_chunk_id != current_chunk_id {
                            linked_chunks_for_this.insert(first_novel_chunk_id);
                        }
                    }
                    // Also link to any other chunks that contain this word
                    if let Some(other_chunks) = self.word_to_chunks.get(&word) {
                        for &other_chunk_id in other_chunks {
                            if other_chunk_id != current_chunk_id {
                                linked_chunks_for_this.insert(other_chunk_id);
                            }
                        }
                    }
                }
            }
        }

        let new_chunk = TextChunk {
            id: current_chunk_id,
            source_file: source_file.to_string_lossy().into_owned(),
            content: sentence.to_string(),
            words: chunk_words,
        };

        self.chunks.insert(current_chunk_id, new_chunk);

        // Update word_to_chunks mapping
        for word in self.chunks.get(&current_chunk_id).unwrap().words.iter() {
            self.word_to_chunks.entry(word.clone()).or_insert_with(HashSet::new).insert(current_chunk_id);
        }

        // Establish bidirectional links for this chunk
        for &linked_id in &linked_chunks_for_this {
            self.chunk_links.entry(current_chunk_id).or_insert_with(HashSet::new).insert(linked_id);
            self.chunk_links.entry(linked_id).or_insert_with(HashSet::new).insert(current_chunk_id);
        }

        Ok(())
    }

    // Helper to extract poem body (reused from previous CRQ)
    fn extract_poem_body(&self, file_path: &Path) -> Result<String> {
        let content = fs::read_to_string(file_path)?;
        let lines: Vec<&str> = content.lines().collect();

        let mut fm_end = -1;
        let mut delimiter_count = 0;
        for (i, line) in lines.iter().enumerate() {
            if line.trim() == "---" {
                delimiter_count += 1;
                if delimiter_count == 2 { // Found the end of the front matter
                    fm_end = i as isize;
                    break;
                }
            }
        }

        if fm_end == -1 {
            return Err(anyhow!("No valid YAML front matter end delimiter found in {:?}", file_path));
        }

        let poem_body_lines = &lines[(fm_end + 1) as usize ..];
        Ok(poem_body_lines.join("\n"))
    }

    /// Main processing function for all poems in a directory.
    fn process_all_poems(&mut self, poems_dir: &Path) -> Result<()> {
        // Simple sentence splitter. More robust NLP libraries would be needed for complex cases.
        let sentence_regex = Regex::new(r"([.!?])\s*").unwrap();

        for entry in fs::read_dir(poems_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                // Filter out non-poem markdown files (e.g., CRQs, logs, specific poems)
                if path.file_name().map_or(false, |name| {
                    name == "CRQ_20250822_Fix_Agent_Sonnet.md" ||
                    name == "manager_lament_20250822.md" ||
                    name == "braindump_20250822.md" ||
                    name == "kitkat_break_log_20250822.md" ||
                    name == "CRQ_20250822_Bag_of_Words_and_Lean4_Expressions.md" ||
                    name == "CRQ_20250822_Novelty_Tracking_and_Chunk_Linking.md" // Exclude this new CRQ
                }) {
                    continue;
                }

                match self.extract_poem_body(&path) {
                    Ok(body) => {
                        // Split by sentence and process each
                        for sentence_part in sentence_regex.split(&body) {
                            let trimmed_sentence = sentence_part.trim();
                            if !trimmed_sentence.is_empty() {
                                // Re-add the delimiter for context if needed, or process as is
                                self.process_sentence(trimmed_sentence, &path)?;
                            }
                        }
                    },
                    Err(e) => eprintln!("Error extracting poem body from {:?}: {}", path, e),
                }
            }
        }
        Ok(())
    }
}

/*
// Conceptual main function demonstrating usage:
fn main() -> Result<()> {
    let poems_dir = Path::new("/data/data/com.termux/files/home/storage/github/libminizinc/docs/poems/");
    let mut tracker = NoveltyTracker::new();
    tracker.process_all_poems(poems_dir)?;

    println!("---"Novelty Tracking Results"---");
    println!("Global Vocabulary Size: {}", tracker.global_vocabulary.len());
    println!("Total Chunks Processed: {}", tracker.chunks.len());

    println!("\nNovel Word First Appearances:");
    for (word, &chunk_id) in &tracker.novel_word_first_appearance {
        println!("  '{}' first novel in Chunk ID: {}", word, chunk_id);
    }

    println!("\nChunk Links:");
    for (chunk_id, linked_ids) in &tracker.chunk_links {
        println!("  Chunk {} linked to: {:?}", chunk_id, linked_ids);
    }

    Ok(())
}
*/
