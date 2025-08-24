use anyhow::Result;
use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;

pub fn count_and_print_raw_matches(file_path: &std::path::PathBuf, regex_pattern: &str) -> Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let filter_regex = Regex::new(regex_pattern)?;

    let mut matches_found = 0;

    println!("Searching raw input for pattern: '{}'
", regex_pattern);

    for (i, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        if filter_regex.is_match(&line) {
            matches_found += 1;
            println!("RAW MATCH (Line {}): {}", i + 1, line);
        }
    }

    println!("
Total raw matches found: {}", matches_found);

    Ok(())
}
