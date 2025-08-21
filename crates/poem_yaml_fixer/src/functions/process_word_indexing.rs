// This file contains the process_word_indexing function.
// It is responsible for extracting words from the poem body, creating a unique key for the poem,
// loading/updating the word index, and saving it.

use std::{collections::HashMap, path::PathBuf};
use anyhow::Result;
use serde_yaml;

use crate::functions::save_word_index::{save_word_index, WordIndex};

pub fn process_word_indexing(
    fixed_fm_title: Option<String>,
    extracted_words: Vec<String>,
    path: &PathBuf,
    debug_mode: bool,
) -> Result<()> {
    // Create a unique key for the poem (e.g., from its title)
    let poem_key = fixed_fm_title.unwrap_or_else(|| path.file_stem().unwrap().to_string_lossy().into_owned());

    // Load existing word index or create a new one
    let word_index_path = PathBuf::from("docs/word_index.yaml");
    let mut word_index: WordIndex = if word_index_path.exists() {
        let content = std::fs::read_to_string(&word_index_path)?;
        serde_yaml::from_str(&content)?
    } else {
        WordIndex { poems: HashMap::new() }
    };

    // Update the word index for the current poem
    word_index.poems.insert(poem_key, extracted_words);

    // Save the updated word index
    save_word_index(&word_index, &word_index_path)?;

    if debug_mode {
        println!("\n--- Updated Word Index ---");
        println!("{:?}", word_index);
        println!("--------------------------");
    }

    Ok(())
}
