use anyhow::{Result, anyhow};
use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;
use serde_json::Deserializer;
use asciicast::Entry;
use strip_ansi_escapes::strip;

pub fn check_raw_matches(file_path: &std::path::PathBuf, regex_pattern: &str) -> Result<bool> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let escaped_regex_pattern = regex::escape(regex_pattern);
    let filter_regex = Regex::new(&escaped_regex_pattern)?;

    let mut de = Deserializer::from_reader(reader).into_iter::<serde_json::Value>();

    // Skip the header
    de.next().ok_or_else(|| anyhow!("Missing header"))?;

    for value in de {
        let entry: Entry = serde_json::from_value(value.map_err(|e: serde_json::Error| anyhow!(e))?)?;
        let cleaned_data = String::from_utf8_lossy(&strip(entry.event_data.as_bytes())?).to_string();
        if filter_regex.is_match(&cleaned_data) {
            return Ok(true);
        }
    }
    Ok(false)
}
