use anyhow::Result;
use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;

pub fn check_raw_matches(file_path: &std::path::PathBuf, regex_pattern: &str) -> Result<bool> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let filter_regex = Regex::new(regex_pattern)?;

    for line_result in reader.lines() {
        let line = line_result?;
        if filter_regex.is_match(&line) {
            return Ok(true);
        }
    }
    Ok(false)
}
