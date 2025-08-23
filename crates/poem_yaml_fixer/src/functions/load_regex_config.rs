#![allow(unused_imports)]
#[cfg(test)]
use anyhow::{Result, anyhow};
use std::{fs, path::PathBuf};
use poem_traits::{RegexConfig, RegexEntry}; // Import RegexConfig and RegexEntry
// Removed: use super::regex_patterns; // Import the regex_patterns module

// Removed: pub fn load_regex_config(path: &PathBuf) -> anyhow::Result<RegexConfig> {
// Removed:     let toml_content = fs::read_to_string(path)?;
// Removed:     let config: RegexConfig = toml::from_str(&toml_content)?;
// Removed:     Ok(config)
// Removed: }