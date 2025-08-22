use anyhow::Result;
use std::fs;
use toml;
use crate::functions::types::RegexConfig;

pub fn load_regex_config() -> Result<RegexConfig> {
    let regex_config_str = fs::read_to_string("crates/poem_yaml_fixer/src/regex_patterns.toml")?;
    let regex_config: RegexConfig = toml::from_str(&regex_config_str)?;
    Ok(regex_config)
}
