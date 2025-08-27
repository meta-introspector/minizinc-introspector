use anyhow::Result;
use std::path::PathBuf;
use regex::Regex;

pub fn extract_patterns_from_rust_file(file_path: &PathBuf) -> Result<Vec<String>> {
    let content = std::fs::read_to_string(file_path)?;
    let mut patterns = Vec::new();

    // Regex to find #[poem_function(...)] attributes
    let attr_regex = Regex::new(r"#\[poem_function\((?s:.*?)\)\]")?;
    // Regex to extract pattern = "..." from within the attribute
    let pattern_regex = Regex::new(r#"pattern\s*=\s*"(?P<pattern>[^"]*)""#)?;

    for line in content.lines() {
        if let Some(captures) = attr_regex.captures(line) {
            if let Some(attr_content) = captures.get(0) { // Get the full attribute match
                if let Some(pattern_captures) = pattern_regex.captures(attr_content.as_str()) {
                    if let Some(pattern_match) = pattern_captures.name("pattern") {
                        patterns.push(pattern_match.as_str().to_string());
                    }
                }
            }
        }
    }
    Ok(patterns)
}
