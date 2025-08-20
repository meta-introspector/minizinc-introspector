use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;

pub const MAX_TOTAL_TERMS: usize = 500_000; // Example limit, adjust as needed

// Function to determine if a term is "junk"
pub fn is_junk_term(term: &str) -> bool {
    // Filter by length: too short or too long
    if term.len() < 2 || term.len() > 64 {
        return true;
    }

    // Filter out terms that are purely numerical (e.g., hashes, large numbers)
    if term.parse::<u64>().is_ok() {
        return true;
    }

    // Filter out terms that look like hex hashes (e.g., "a1b2c3d4e5f6")
    if term.len() >= 8 && term.chars().all(|c| c.is_ascii_hexdigit()) {
        return true;
    }

    // Filter out terms with a high proportion of non-alphanumeric characters
    let alphanumeric_chars = term.chars().filter(|c| c.is_alphanumeric()).count();
    if alphanumeric_chars * 2 < term.len() { // Less than 50% alphanumeric
        return true;
    }

    // Filter out common build artifacts or mangled names
    if term.contains("__") || term.contains("::") || term.contains("target") || term.contains("debug") || term.contains("release") || term.contains("build") {
        return true;
    }

    // Filter out terms that are just single characters or common placeholders
    if term.len() == 1 && !term.chars().next().unwrap().is_alphabetic() {
        return true;
    }

    false
}

pub fn load_and_filter_terms(hierarchical_index_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let hierarchical_term_index_file = PathBuf::from(hierarchical_index_path);

    println!("cargo:warning=Loading hierarchical term index from: {:?}", hierarchical_term_index_file);
    let cached_data = fs::read_to_string(&hierarchical_term_index_file)?;
    let hierarchical_term_index: HashMap<String, HashMap<PathBuf, usize>> = serde_json::from_str(&cached_data)?;

    println!("cargo:warning=Successfully loaded index with {} terms.", hierarchical_term_index.len());

    let filtered_terms: Vec<String> = hierarchical_term_index.keys()
        .cloned()
        .filter(|term| !is_junk_term(term)) // Apply the junk term filter
        .collect();

    println!("cargo:warning=Filtered down to {} non-junk terms.", filtered_terms.len());

    if filtered_terms.len() > MAX_TOTAL_TERMS {
        return Err(format!("Error: Total filtered terms ({}) exceeds MAX_TOTAL_TERMS ({})", filtered_terms.len(), MAX_TOTAL_TERMS).into());
    }
    Ok(filtered_terms)
}