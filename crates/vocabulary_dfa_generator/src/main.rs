use std::fs::File;
use std::io::{self, BufReader, BufRead, Write};
use std::path::PathBuf;
use std::collections::HashMap;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct BuildPathsConfig {
    home_dir: String,
    github_root: String,
    project_root: String,
    hierarchical_term_index: String,
}

const MAX_TERMS_PER_FILE: usize = 1000;

fn sanitize_filename_char(c: char) -> String {
    if c.is_ascii_alphanumeric() {
        c.to_string()
    } else {
        format!("U{:04X}", c as u32) // Format as UXXXX (hex Unicode codepoint)
    }
}

fn sanitize_filename(s: &str) -> String {
    s.chars().map(|c| sanitize_filename_char(c)).collect::<Vec<String>>().join("")
}

fn is_purely_numeric(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_digit())
}

fn is_purely_hex(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_hexdigit())
}

fn main() -> io::Result<()> {
    let config_path = PathBuf::from("/data/data/com.termux/files/home/storage/github/libminizinc/build_config.toml");
    let config_content = fs::read_to_string(&config_path)?;
    let config: BuildPathsConfig = toml::from_str(&config_content)?;

    let input_file_path = config.project_root.clone() + "all_terms.txt";
    let base_output_dir = PathBuf::from(config.project_root.clone() + "crates/vocabulary_dfa_lib/src");

    // Create the base output directory if it doesn't exist
    std::fs::create_dir_all(&base_output_dir)?;

    let file = File::open(input_file_path)?;
    let reader = BufReader::new(file);

    let mut terms_by_first_letter: HashMap<char, Vec<String>> = HashMap::new();

    for line in reader.lines() {
        let term = line?;
        if term.is_empty() { continue; }

        let first_char = term.chars().next().unwrap();

        // Filter: Must start with an alphabetic character
        if !first_char.is_ascii_alphabetic() { continue; }

        // Filter: Must not be purely numeric or purely hex
        if is_purely_numeric(&term) || is_purely_hex(&term) { continue; }

        terms_by_first_letter.entry(first_char.to_ascii_lowercase())
            .or_insert_with(Vec::new)
            .push(term);
    }

    for (first_letter, terms) in terms_by_first_letter {
        if terms.len() > MAX_TERMS_PER_FILE {
            // Split by second letter
            let mut terms_by_second_letter: HashMap<char, Vec<String>> = HashMap::new();
            for term in terms {
                if let Some(second_char) = term.chars().nth(1) {
                    terms_by_second_letter.entry(second_char.to_ascii_lowercase())
                        .or_insert_with(Vec::new)
                        .push(term);
                } else {
                    // Handle single-character words that were part of a large first-letter group
                    // For now, add them to a special '_' group or the main group if no second char
                    terms_by_second_letter.entry('_')
                        .or_insert_with(Vec::new)
                        .push(term);
                }
            }

            for (second_letter, sub_terms) in terms_by_second_letter {
                let sanitized_first_letter = sanitize_filename(&first_letter.to_string());
                let sanitized_second_letter = sanitize_filename(&second_letter.to_string());

                let sub_dir = base_output_dir.join(&sanitized_first_letter);
                std::fs::create_dir_all(&sub_dir)?;

                let file_name = format!("{}{}_dfa.rs", sanitized_first_letter, sanitized_second_letter);
                let output_file_path = sub_dir.join(&file_name);
                let mut output_file = File::create(output_file_path)?;

                writeln!(output_file, "use regex::Regex;")?;
                writeln!(output_file, "")?;
                writeln!(output_file, "pub fn matches_{}{}(text: &str) -> bool {{", first_letter, second_letter)?;
                writeln!(output_file, "    let pattern = r\"^({})$\";", sub_terms.join("|"))?;
                writeln!(output_file, "    let re = Regex::new(pattern).unwrap();")?;
                writeln!(output_file, "    re.is_match(text)")?;
                writeln!(output_file, "}}")?;
            }
        } else {
            // Write directly to a single file for this first letter
            let sanitized_first_letter = sanitize_filename(&first_letter.to_string());

            let sub_dir = base_output_dir.join(&sanitized_first_letter);
            std::fs::create_dir_all(&sub_dir)?;

            let file_name = format!("{}_dfa.rs", sanitized_first_letter);
            let output_file_path = sub_dir.join(&file_name);
            let mut output_file = File::create(output_file_path)?;

            writeln!(output_file, "use regex::Regex;")?;
            writeln!(output_file, "")?;
            writeln!(output_file, "pub fn matches_{}(text: &str) -> bool {{", first_letter)?;
            writeln!(output_file, "    let pattern = r\"^({})$\";", terms.join("|"))?;
            writeln!(output_file, "    let re = Regex::new(pattern).unwrap();")?;
            writeln!(output_file, "    re.is_match(text)")?;
            writeln!(output_file, "}}")?;
        }
    }

    println!("DFA modules generated successfully in {}", base_output_dir.display());

    Ok(())
}
