


//use std::collections::HashMap;
//use std::path::PathBuf;

use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;
use crate::functions::save_word_index::save_word_index;
use crate::functions::types::WordIndex;

#[test]

#[test]
fn test_save_word_index() -> anyhow::Result<()> {
    // 1. Create a dummy WordIndex instance
    let mut word_index = WordIndex {
        poems: HashMap::new(),
    };
    word_index.poems.insert(
        "poem1".to_string(),
        vec![
            ("word1".to_string(), 1),
            ("word2".to_string(), 2),
        ].into_iter().collect(),
    );
    word_index.poems.insert(
        "poem2".to_string(),
        vec![
            ("word3".to_string(), 3),
            ("word4".to_string(), 4),
        ].into_iter().collect(),
    );

    // 2. Define a temporary file path
    let temp_dir = tempfile::tempdir()?;
    let temp_file_path = temp_dir.path().join("word_index.yaml");

    // 3. Call save_word_index
    save_word_index(&word_index, &temp_file_path)?;

    // 4. Read the saved file and assert its content matches the dummy WordIndex
    let saved_content = fs::read_to_string(&temp_file_path)?;
    let loaded_word_index: WordIndex = serde_yaml::from_str(&saved_content)?;

    assert_eq!(word_index.poems.len(), loaded_word_index.poems.len());
    assert_eq!(word_index.poems.get("poem1").unwrap().len(), loaded_word_index.poems.get("poem1").unwrap().len());
    assert_eq!(word_index.poems.get("poem2").unwrap().len(), loaded_word_index.poems.get("poem2").unwrap().len());

    assert_eq!(*word_index.poems.get("poem1").unwrap().get("word1").unwrap(), *loaded_word_index.poems.get("poem1").unwrap().get("word1").unwrap());
    assert_eq!(*word_index.poems.get("poem1").unwrap().get("word2").unwrap(), *loaded_word_index.poems.get("poem1").unwrap().get("word2").unwrap());
    assert_eq!(*word_index.poems.get("poem2").unwrap().get("word3").unwrap(), *loaded_word_index.poems.get("poem2").unwrap().get("word3").unwrap());
    assert_eq!(*word_index.poems.get("poem2").unwrap().get("word4").unwrap(), *loaded_word_index.poems.get("poem2").unwrap().get("word4").unwrap());

    // Clean up temporary directory
    fs::remove_dir_all(&temp_dir)?;

    Ok(())
}
