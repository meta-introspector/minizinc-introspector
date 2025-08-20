use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;
use regex::Regex; // For building the regex

const HIERARCHICAL_INDEX_PATH: &str = "/data/data/com.termux/files/home/storage/github/hierarchical_term_index.json";
const GENERATED_RECOGNIZER_PATH: &str = "src/generated_recognizer.rs";

// Function to determine if a term is "junk"
fn is_junk_term(term: &str) -> bool {
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

// This function will now return the regex pattern and the filtered terms
pub fn run_compile_terms_regex() -> Result<(String, Vec<String>), Box<dyn std::error::Error>> {
    println!("ZOS Fast Query Tool - Compiling Terms to Regex");

    let hierarchical_term_index_file = PathBuf::from(HIERARCHICAL_INDEX_PATH);

    println!("Loading hierarchical term index from: {:?}", hierarchical_term_index_file);
    let cached_data = fs::read_to_string(&hierarchical_term_index_file)?;
    let hierarchical_term_index: HashMap<String, HashMap<PathBuf, usize>> = serde_json::from_str(&cached_data)?;

    println!("Successfully loaded index with {} terms.", hierarchical_term_index.len());

    let filtered_terms: Vec<String> = hierarchical_term_index.keys()
        .cloned()
        .filter(|term| !is_junk_term(term)) // Apply the junk term filter
        .collect();

    println!("Filtered down to {} non-junk terms.", filtered_terms.len());

    // Escape terms for regex and join them with '|'
    let regex_pattern = filtered_terms.iter() // Use iter() here to avoid consuming filtered_terms
        .map(|s| regex::escape(&s))
        .collect::<Vec<String>>()
        .join("|");

    // Create a Regex object to validate the pattern (optional, but good practice)
    let _ = Regex::new(&regex_pattern)?;

    Ok((regex_pattern, filtered_terms)) // Return the regex pattern and filtered terms
}

// New function to generate the Rust recognizer file
pub fn generate_recognizer_file(regex_pattern: String, terms: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut file_content = String::new();
    file_content.push_str("use regex::Regex;\n");
    file_content.push_str("use std::collections::HashSet;\n");
    file_content.push_str("use lazy_static::lazy_static;\n\n"); // Add lazy_static import

    file_content.push_str("lazy_static! {\n");
    file_content.push_str("    pub static ref RECOGNIZER_REGEX: Regex = {\n");
    file_content.push_str(&format!("        Regex::new(r\"{{}}\").expect(\"Failed to compile regex\")\n", regex_pattern));
    file_content.push_str("    };\n");
    file_content.push_str("}\\n\n");

    file_content.push_str("lazy_static! {\n");
    file_content.push_str("    pub static ref RECOGNIZER_TERMS: HashSet<String> = {\n");
    file_content.push_str("        let mut set = HashSet::new();\n");
    for term in terms {
        file_content.push_str(&format!("        set.insert(\"{}\".to_string());\n", term));
    }
    file_content.push_str("        set\n");
    file_content.push_str("    };\n");
    file_content.push_str("}");

    fs::write(GENERATED_RECOGNIZER_PATH, file_content)?;
    println!("Generated recognizer file at: {}", GENERATED_RECOGNIZER_PATH);

    Ok(())
}