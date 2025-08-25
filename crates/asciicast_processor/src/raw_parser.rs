use anyhow::{Result, anyhow};
use std::fs::File;
use std::io::{BufReader,
	      //BufRead
};
use regex::Regex;
use serde_json::Deserializer;
use asciicast::Entry;
use strip_ansi_escapes::strip;
use gemini_utils::gemini_eprintln;
use gemini_utils::string_processor::clean_non_ascii;

pub fn check_raw_matches(file_path: &std::path::PathBuf, regex_pattern: &str) -> Result<bool> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let escaped_regex_pattern = regex::escape(regex_pattern);
    let filter_regex = Regex::new(&escaped_regex_pattern)?;

    gemini_eprintln!("DEBUG (raw_parser): Original Regex Pattern: :regex_pattern:", regex_pattern = regex_pattern);
    gemini_eprintln!("DEBUG (raw_parser): Escaped Regex Pattern: :escaped_regex_pattern:", escaped_regex_pattern = escaped_regex_pattern);

    let mut de = Deserializer::from_reader(reader).into_iter::<serde_json::Value>();

    // Skip the header
    de.next().ok_or_else(|| anyhow!("Missing header"))?;

    for value in de {
        let entry: Entry = serde_json::from_value(value.map_err(|e: serde_json::Error| anyhow!(e))?)?;
        let cleaned_data = clean_non_ascii(&String::from_utf8_lossy(&strip(entry.event_data.as_bytes())?).to_string());
        gemini_eprintln!("DEBUG (raw_parser): Matching against cleaned data: :cleaned_data:", cleaned_data = cleaned_data);
        if filter_regex.is_match(&cleaned_data) {
            gemini_eprintln!("DEBUG (raw_parser): Match found in raw data!");
            return Ok(true);
        }
    }
    gemini_eprintln!("DEBUG (raw_parser): No match found in raw data.");
    Ok(false)
}
