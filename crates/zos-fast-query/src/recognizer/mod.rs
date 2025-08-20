use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use serde_json;

include!(concat!(env!("OUT_DIR"), "/generated_recognizer.rs"));

pub struct TermRecognizer {
    terms: HashMap<String, HashSet<String>>,
}

impl TermRecognizer {
    /// Creates a new TermRecognizer. It now loads terms from generated files.
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut terms_map = HashMap::new();
        let out_dir = std::env::var("OUT_DIR")?;

        for file_name in RECOGNIZER_TERM_FILES {
            let file_path = PathBuf::from(&out_dir).join(file_name);
            let json_content = std::fs::read_to_string(&file_path)?;
            let terms_in_chunk: Vec<String> = serde_json::from_str(&json_content)?;

            // Extract the sanitized char from the filename (e.g., "terms_a_0.json" -> "a")
            let parts: Vec<&str> = file_name.split('_').collect();
            if parts.len() >= 2 {
                let sanitized_char = parts[1].to_string();
                terms_map.entry(sanitized_char).or_insert_with(HashSet::new).extend(terms_in_chunk);
            }
        }

        Ok(Self { terms: terms_map })
    }

    /// Recognizes terms in the given text and returns a vector of matched terms.
    pub fn recognize_terms(&self, text: &str) -> Vec<String> {
        let mut matched_terms = Vec::new();
        // This is a simplified recognition. A real implementation might need a more advanced DFA/Aho-Corasick.
        // For now, we iterate through words and check if they exist in our loaded terms.
        for word in text.split_whitespace() {
            if let Some(first_char) = word.chars().next() {
                let sanitized_char = crate::build_utils::sanitize_filename(&first_char.to_string());
                if let Some(terms_for_char) = self.terms.get(&sanitized_char) {
                    if terms_for_char.contains(word) {
                        matched_terms.push(word.to_string());
                    }
                }
            }
        }
        matched_terms
    }
}
