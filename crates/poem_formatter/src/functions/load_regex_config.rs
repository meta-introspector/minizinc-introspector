use anyhow::Result;
use std::fs;
use toml;
use crate::functions::types::RegexConfig;

pub fn load_regex_config() -> Result<RegexConfig> {
    let regex_config_str = fs::read_to_string("crates/poem_formatter/src/regex_patterns.toml")?; //FIXME change to toml config
    let regex_config: RegexConfig = toml::from_str(&regex_config_str)?;
    Ok(regex_config)
}
