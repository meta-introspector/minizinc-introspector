//! external_config.rs
//!
//! This module handles the loading of external regex configuration files
//! and merging them with the default configuration.
//!
//! External configuration files allow users to customize or extend the
//! regex patterns used by `poem_yaml_fixer` without modifying the
//! application's source code.
use poem_traits::RegexConfig;
use anyhow::{Result
	     //, anyhow, Error
};
use std::{fs, path::PathBuf};
//use poem_traits::{RegexConfig, RegexEntry}; // Import necessary structs from poem_traits
use toml; // Import toml crate for parsing

/// Loads a `RegexConfig` from a specified TOML file.
///
/// This function reads the content of a TOML file at the given `path`,
/// parses it into a `RegexConfig` struct, and returns the result.
///
/// # Arguments
///
/// * `path` - A reference to a `PathBuf` indicating the path to the TOML file.
///
/// # Returns
///
/// A `Result` which is `Ok(RegexConfig)` if the file is successfully read
/// and parsed, or an `Err` if an error occurs during file operations or TOML parsing.
pub fn load_regex_config(path: &PathBuf) -> Result<RegexConfig> {
    let toml_content = fs::read_to_string(path)?;
    let config: RegexConfig = toml::from_str(&toml_content)?;
    Ok(config)
}

/// Merges an external `RegexConfig` into a mutable default `RegexConfig`.
///
/// This function iterates through the `regexes` in the `external_config`.
/// If an entry with the same `name` already exists in the `default_config`,
/// it updates that entry. Otherwise, it adds the new entry to the `default_config`.
///
/// # Arguments
///
/// * `default_config` - A mutable reference to the `RegexConfig` to be updated.
/// * `external_config` - The `RegexConfig` containing external entries to merge.
///
/// # Examples
///
/// ```
/// // Assuming default_config and external_config are initialized RegexConfig instances
/// // merge_external_config(&mut default_config, external_config);
/// ```
pub fn merge_external_config(default_config: &mut RegexConfig, external_config: RegexConfig) {
    for external_entry in external_config.regexes {
        if let Some(default_entry) = default_config.regexes.iter_mut().find(|e| e.name == external_entry.name) {
            *default_entry = external_entry;
        } else {
            default_config.regexes.push(external_entry);
        }
    }
}
