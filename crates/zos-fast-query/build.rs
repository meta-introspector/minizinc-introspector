use std::env;
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;
use regex::Regex; // For building the regex
use std::io::Write; // For writeln!

const HIERARCHICAL_INDEX_PATH: &str = "/data/data/com.termux/files/home/storage/github/hierarchical_term_index.json";

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Rerun this build script if the hierarchical_term_index.json changes
    println!("cargo:rerun-if-changed={}", HIERARCHICAL_INDEX_PATH);

    println!("cargo:warning=ZOS Fast Query Tool - Compiling Terms to Regex (Build Script)");

    let hierarchical_term_index_file = PathBuf::from(HIERARCHICAL_INDEX_PATH);

    println!("cargo:warning=Loading hierarchical term index from: {:?}", hierarchical_term_index_file);
    let cached_data = fs::read_to_string(&hierarchical_term_index_file)?;
    let hierarchical_term_index: HashMap<String, HashMap<PathBuf, usize>> = serde_json::from_str(&cached_data)?;

    println!("cargo:warning=Successfully loaded index with {} terms.", hierarchical_term_index.len());

    let filtered_terms: Vec<String> = hierarchical_term_index.keys()
        .cloned()
        .filter(|term| !is_junk_term(term)) // Apply the junk term filter
        .collect();

    println!("cargo:warning=Filtered down to {} non-junk terms.", filtered_terms.len());

    // Escape terms for regex and join them with '|'
    let regex_pattern = filtered_terms.iter()
        .map(|s| regex::escape(&s))
        .collect::<Vec<String>>()
        .join("|");

    // Create a Regex object to validate the pattern (optional, but good practice)
    let _ = Regex::new(&regex_pattern)?;

    // Generate the Rust recognizer file
    let out_dir = env::var("OUT_DIR")?;
    let dest_path = PathBuf::from(&out_dir).join("generated_recognizer.rs"); // Name relative to OUT_DIR

    let mut file = fs::File::create(&dest_path)?;

    writeln!(file, "use regex::Regex;")?;
    writeln!(file, "use std::collections::HashSet;")?;
    writeln!(file, "use lazy_static::lazy_static;\n")?;

    writeln!(file, "lazy_static! {{")?;
    writeln!(file, "    pub static ref RECOGNIZER_REGEX: Regex = {{")?;
    writeln!(file, "        Regex::new(r#\"{}\"#).expect(\"Failed to compile regex\")", regex_pattern)?;
    writeln!(file, "    }};")?;
    writeln!(file, "}}
")?;

    writeln!(file, "lazy_static! {{")?;
    writeln!(file, "    pub static ref RECOGNIZER_TERMS: HashSet<String> = {{")?;
    writeln!(file, "        let mut set = HashSet::new();")?;
    for term in filtered_terms { // Use filtered_terms here
        writeln!(file, "        set.insert(\"{}\".to_string());", term)?;
    }
    writeln!(file, "        set")?;
    writeln!(file, "    }};")?;
    writeln!(file, "}}")?;

    println!("cargo:warning=Generated recognizer file at: {:?}", dest_path);

    Ok(())
}