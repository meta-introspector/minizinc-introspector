// This file contains the extract_words_from_text function.
// It extracts alphanumeric words from a given text string.

#[allow(dead_code)]
pub fn extract_words_from_text(text: &str) -> Vec<String> {
    text.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}
