use std::fs::File;
use std::io::{self, BufReader, BufRead, Write};
use std::collections::HashMap;

const MAX_TERMS_PER_FILE: usize = 1000;

fn is_purely_numeric(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_digit())
}

fn is_purely_hex(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_hexdigit())
}

fn main() -> io::Result<()> {
    let input_file_path = "/data/data/com.termux/files/home/storage/github/libminizinc/all_terms.txt";
    let output_dir = "/data/data/com.termux/files/home/storage/github/libminizinc/crates/vocabulary_dfa_lib/src";

    // Create the output directory if it doesn't exist
    std::fs::create_dir_all(output_dir)?;

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
                let file_name = format!("{}{}_dfa.rs", first_letter, second_letter);
                let output_file_path = format!("{}/{}", output_dir, file_name);
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
            let file_name = format!("{}_dfa.rs", first_letter);
            let output_file_path = format!("{}/{}", output_dir, file_name);
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

    println!("DFA modules generated successfully in {}", output_dir);

    Ok(())
}
