#[cfg(test)]
use anyhow::{Result, anyhow};
use std::{fs, path::PathBuf};
use poem_traits::{RegexConfig, RegexEntry}; // Import RegexConfig and RegexEntry
use super::regex_patterns; // Import the regex_patterns module

pub fn get_default_regex_config() -> RegexConfig {
    RegexConfig {
        regexes: vec![
            RegexEntry {
                name: "old_meme_regex".to_string(),
                pattern: regex_patterns::OLD_MEME_REGEX_PATTERN.to_string(),
                callback_function: "handle_old_meme_regex".to_string(),
            },
            RegexEntry {
                name: "new_meme_desc_regex".to_string(),
                pattern: regex_patterns::NEW_MEME_DESC_REGEX_PATTERN.to_string(),
                callback_function: "handle_new_meme_desc_regex".to_string(),
            },
            RegexEntry {
                name: "new_meme_template_regex".to_string(),
                pattern: regex_patterns::NEW_MEME_TEMPLATE_REGEX_PATTERN.to_string(),
                callback_function: "handle_new_meme_template_regex".to_string(),
            },
            RegexEntry {
                name: "title_regex".to_string(),
                pattern: regex_patterns::TITLE_REGEX_PATTERN_FROM_TOML.to_string(),
                callback_function: "handle_title_regex".to_string(),
            },
            RegexEntry {
                name: "summary_regex".to_string(),
                pattern: regex_patterns::SUMMARY_REGEX_PATTERN.to_string(),
                callback_function: "handle_summary_regex".to_string(),
            },
            RegexEntry {
                name: "keywords_regex".to_string(),
                pattern: regex_patterns::KEYWORDS_REGEX_PATTERN_FROM_TOML.to_string(),
                callback_function: "handle_keywords_regex".to_string(),
            },
            RegexEntry {
                name: "emojis_regex".to_string(),
                pattern: regex_patterns::EMOJIS_REGEX_PATTERN.to_string(),
                callback_function: "handle_emojis_regex".to_string(),
            },
            RegexEntry {
                name: "art_generator_instructions_regex".to_string(),
                pattern: regex_patterns::ART_GENERATOR_INSTRUCTIONS_REGEX_PATTERN.to_string(),
                callback_function: "handle_art_generator_instructions_regex".to_string(),
            },
            RegexEntry {
                name: "poem_body_start_regex".to_string(),
                pattern: regex_patterns::POEM_BODY_START_REGEX_PATTERN.to_string(),
                callback_function: "handle_poem_body_start_regex".to_string(),
            },
        ],
    }
}

pub fn load_regex_config(path: &PathBuf) -> anyhow::Result<RegexConfig> {
    let toml_content = fs::read_to_string(path)?;
    let config: RegexConfig = toml::from_str(&toml_content)?;
    Ok(config)
}
