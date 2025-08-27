use std::collections::HashMap;
use regex::Regex;
use stop_words::get;

pub fn clean_and_tokenize(text: &str) -> Vec<String> {
    let text = text.to_lowercase();
    let re = Regex::new(r"[^a-z0-9\s]").unwrap(); // Remove punctuation
    let cleaned_text = re.replace_all(&text, "").to_string();
    
    cleaned_text.split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

pub fn filter_stop_words(words: Vec<String>) -> Vec<String> {
    let stopwords: Vec<String> = get(stop_words::LANGUAGE::English).iter().map(|s| s.to_string()).collect();
    words.into_iter()
        .filter(|word| !stopwords.contains(word))
        .collect()
}

pub fn generate_bag_of_words(words: Vec<String>) -> HashMap<String, usize> {
    let mut bow = HashMap::new();
    for word in words {
        *bow.entry(word).or_insert(0) += 1;
    }
    bow
}
