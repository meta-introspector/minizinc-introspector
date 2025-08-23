//! default_config.rs
//!
//! This module provides the default regex configuration for `poem_yaml_fixer`.
//! It defines a `RegexConfig` struct containing a predefined set of `RegexEntry` objects.
//! These entries represent the initial set of regex patterns and their associated callback functions
//! that the `poem_yaml_fixer` uses to process Markdown files.
//!
//! This configuration serves as a baseline and can be extended or overridden by external
//! configuration files.

use poem_traits::{RegexConfig, RegexEntry}; // Import necessary structs from poem_traits
use super::super::regex_patterns; // Import the regex_patterns module from the parent directory

/// Provides the default `RegexConfig` for the `poem_yaml_fixer`.
///
/// This function initializes a `RegexConfig` with a hardcoded set of
/// `RegexEntry` objects. Each `RegexEntry` defines a regex pattern,
/// a name for the pattern, and the name of the callback function
/// that should be executed when the pattern matches.
///
/// These default patterns are essential for the basic operation of the
/// `poem_yaml_fixer` and serve as a starting point for processing
/// various elements within Markdown files, especially YAML front matter.
///
/// # Returns
///
/// A `RegexConfig` instance populated with default regex entries.
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
