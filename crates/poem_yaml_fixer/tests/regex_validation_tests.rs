use anyhow::Result;
use regex::Regex;
use std::fs;
use poem_traits::RegexConfig;
use poem_yaml_fixer::functions::regex_patterns::{KEYWORDS_REGEX, MALFORMED_MEME_REGEX, UNQUOTED_COLON_REGEX, DOCUMENT_SEPARATOR_REGEX};

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

#[test]
fn test_regex_patterns_constants_compile() {
    // Test compilation of each constant regex pattern
    let _ = &KEYWORDS_REGEX;
    let _ = &MALFORMED_MEME_REGEX;
    let _ = &UNQUOTED_COLON_REGEX;
    let _ = &DOCUMENT_SEPARATOR_REGEX;

    // Add specific test cases for each regex constant if needed
    // Example for DOCUMENT_SEPARATOR_REGEX:
    assert!(DOCUMENT_SEPARATOR_REGEX.is_match("---"));
    assert!(!DOCUMENT_SEPARATOR_REGEX.is_match("----"));
    assert!(!DOCUMENT_SEPARATOR_REGEX.is_match("---abc"));
}
