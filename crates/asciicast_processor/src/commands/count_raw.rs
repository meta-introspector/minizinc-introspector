use anyhow::Result;
use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;

use crate::cli::CountRawArgs;
use gemini_utils::gemini_eprintln;

pub fn handle_count_raw_command(args: &CountRawArgs) -> Result<()> {
    let CountRawArgs { regex, input_file } = args;

    let file = File::open(input_file)?;
    let reader = BufReader::new(file);
    let escaped_regex = regex::escape(&regex);
    let filter_regex = Regex::new(&escaped_regex)?;

    let mut matches_found = 0;
  
    gemini_eprintln!("Searching raw input for pattern: ':regex::newline:", regex = regex);

    for (i, line_result) in reader.lines().enumerate() {
        let line = line_result?.to_string(); // Convert to String
        if filter_regex.is_match(&line) {
            matches_found += 1;
            gemini_eprintln!("RAW MATCH (Line :line_num:): :line:", line_num = i + 1, line = line);
        }
    }
    gemini_eprintln!("::newline:: Total raw matches found: :matches_found:", matches_found = matches_found);

    Ok(())
}
