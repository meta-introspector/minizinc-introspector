use anyhow::Result;
use regex::Regex;
use std::fs;
//use std::path::PathBuf;
use poem_traits::RegexConfig; // Assuming RegexConfig is public and accessible

#[test]
fn test_all_configured_regexes_compile() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let regex_config_path = current_dir.join("regex_config.toml");

    // Ensure the regex_config.toml exists
    assert!(regex_config_path.exists(), "regex_config.toml not found at {:?}", regex_config_path);

    let config_content = fs::read_to_string(&regex_config_path)?;
    let regex_config: RegexConfig = toml::from_str(&config_content)?;

    for entry in regex_config.regexes {
        println!("Testing regex: \"{}\" for name: \"{}\"", entry.pattern, entry.name);
        // Attempt to compile the regex
        let compiled_regex = Regex::new(&entry.pattern);
        assert!(compiled_regex.is_ok(), "Failed to compile regex \"{}\" for name \"{}\": {:?}", entry.pattern, entry.name, compiled_regex.unwrap_err());
    }

    Ok(())
}

// Optional: Add more specific tests for individual regexes if needed
// For example, for the "document_separator" regex:
#[test]
fn test_document_separator_regex() {
    let regex = Regex::new(r"^---").unwrap();

    // Positive test cases
    assert!(regex.is_match("---"));
    assert!(regex.is_match("---")); // Even with leading/trailing whitespace if trim is applied before matching

    // Negative test cases
    assert!(!regex.is_match("----"));
    assert!(!regex.is_match("---abc"));
    assert!(!regex.is_match("abc---"));
    assert!(!regex.is_match(" ---"));
    assert!(!regex.is_match("--- "));
    assert!(!regex.is_match(""));
    assert!(!regex.is_match("some text"));
}
