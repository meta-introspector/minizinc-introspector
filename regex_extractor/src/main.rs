use walkdir::WalkDir;
use regex::Regex;
use std::fs;
use std::path::PathBuf;
use std::collections::HashSet;

fn extract_old_regexes(file_path: &PathBuf) -> anyhow::Result<HashSet<String>> {
    let mut old_regexes = HashSet::new();
    let content = fs::read_to_string(file_path)?;
    let re = Regex::new(r"pub const \w+: &str = (.*);").unwrap();

    for line in content.lines() {
        if let Some(caps) = re.captures(line) {
            if let Some(pattern_with_quotes) = caps.get(1) {
                let mut pattern = pattern_with_quotes.as_str().to_string();
                // Strip leading r#" and trailing "#
                if pattern.starts_with("r#\"") && pattern.ends_with("\"#") {
                    pattern = pattern[3..pattern.len() - 2].to_string();
                } else if pattern.starts_with("\"") && pattern.ends_with("\"") {
                    pattern = pattern[1..pattern.len() - 1].to_string();
                }
                old_regexes.insert(pattern);
            }
        }
    }
    Ok(old_regexes)
}

fn main() -> anyhow::Result<()> {
    let poems_dir = PathBuf::from("/data/data/com.termux/files/home/storage/github/libminizinc/docs/poems/");
    let regex_patterns_file = PathBuf::from("/data/data/com.termux/files/home/storage/github/libminizinc/crates/poem_yaml_fixer/src/functions/regex_patterns.rs");

    let mut new_regexes: HashSet<String> = HashSet::new();

    // Regex to find any string literal (content within "..." or r#"..."#)
    let string_literal_regex = Regex::new(r#"(?:r#"(.*?)"|"(.*?)")"#).unwrap();
    // Regex to check if a string contains common regex metacharacters
    let metachar_check_regex = Regex::new(r".*[.*+?|(){}\[\]^$\\]+.*\").unwrap();

    for entry in WalkDir::new(&poems_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().is_some_and(|ext| ext == "md") {
            let content = fs::read_to_string(path)?;
            for line in content.lines() {
                for cap in string_literal_regex.captures_iter(line) {
                    if let Some(matched_literal) = cap.get(1).or_else(|| cap.get(2)) {
                        let extracted_string = matched_literal.as_str().to_string();
                        if metachar_check_regex.is_match(&extracted_string) {
                            new_regexes.insert(extracted_string);
                        }
                    }
                }
            }
        }
    }

    let old_regexes = extract_old_regexes(&regex_patterns_file)?;

    println!("---
Regex Comparison Report---");
    println!("\nOld Regexes (from regex_patterns.rs):");
    for regex in &old_regexes {
        println!("- {}", regex);
    }

    println!("\nNew Regexes (from poems):");
    for regex in &new_regexes {
        println!("- {}", regex);
    }

    println!("\nNew Regexes already present in Old Regexes:");
    for regex in &new_regexes {
        if old_regexes.contains(regex) {
            println!("- {}", regex);
        }
    }

    println!("\nNew Regexes NOT present in Old Regexes:");
    for regex in &new_regexes {
        if !old_regexes.contains(regex) {
            println!("- {}", regex);
        }
    }

    Ok(())
}
