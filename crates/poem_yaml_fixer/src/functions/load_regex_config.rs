use anyhow::Result;
use crate::functions::types::RegexConfig;

pub fn load_regex_config() -> Result<RegexConfig> {
    // Dummy implementation for now
    println!("Loading regex config.");
    Ok(RegexConfig { regexes: Vec::new() })
}