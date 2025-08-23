use anyhow::Result;
use std::path::{Path, PathBuf};
use std::io::Write;
use grex::RegExpBuilder;
use regex::Regex;

pub fn process_unmatched_lines_for_grex(
    unmatched_lines: &[String],
    file_path: &Path,
    current_dir: &Path,
    log_dir: &PathBuf,
) -> Result<()> {
    if unmatched_lines.is_empty() {
        return Ok(())
    }

    let poem_file_stem = file_path.file_stem().and_then(|s| s.to_str()).unwrap_or("unknown_poem");
    let alphanumeric_regex = Regex::new(r"[^a-zA-Z0-9_]+")?;

    println!("\n--- Suggested Regexes for Unmatched Lines from {:?} ---", file_path);

    for (i, line) in unmatched_lines.iter().enumerate() {
        let generated_regex_pattern = RegExpBuilder::from(&[line]).build();

        let sanitized_line = alphanumeric_regex.replace_all(line, "_").to_string();
        let unique_id = if sanitized_line.len() > 30 {
            &sanitized_line[0..30]
        } else {
            &sanitized_line
        };

        let suggested_name = format!("grex_generated_{}_{}", poem_file_stem, unique_id);

        println!("\nSuggested Regex {}:", i);
        println!("  Name: {}", suggested_name);
        println!("  Pattern: \"{}\"", generated_regex_pattern);
        println!("  Callback Function: handle_{}_regex", suggested_name);
        println!("  To add this to regex_patterns.toml, add the following:");
        println!("  --------------------------------------------------");
        println!("  [[regexes]]");
        println!("  name = \"{}\"", suggested_name);
        println!("  pattern = \"{}\"", generated_regex_pattern);
        println!("  callback_function = \"handle_{}_regex\"", suggested_name);
        println!("  --------------------------------------------------");
    }

    println!("\n--- End of Suggested Regexes ---");

    Ok(())
}


