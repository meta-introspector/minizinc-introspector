use anyhow::{Result, anyhow};
use std::{fs, path::PathBuf};
use poem_traits::RegexConfig; // Import RegexConfig directly

pub fn load_regex_config(config_path: &PathBuf) -> Result<RegexConfig> {
    println!("Loading regex config from: {:?}", config_path);
    let config_content = fs::read_to_string(config_path)
        .map_err(|e| anyhow!("Failed to read regex config file {:?}: {}", config_path, e))?;
    let regex_config: RegexConfig = toml::from_str(&config_content)
        .map_err(|e| anyhow!("Failed to parse regex config TOML from {:?}: {}", config_path, e))?;
    Ok(regex_config)
}