//! registry_populator.rs
//!
//! This module provides functionality to populate the `PoemFunctionRegistry`
//! with `PoemFunctionMetadata` derived from a `RegexConfig`.
//!
//! This is crucial for ensuring that all regex patterns, whether from
//! default configurations or external TOML files, are correctly
//! registered and available for matching and callback execution.

use anyhow::{Result, anyhow};
use poem_traits::{PoemFunctionMetadata, RegexConfig, PoemFrontMatterTrait}; // Import PoemFrontMatterTrait
use crate::functions::types::PoemFunctionRegistry;

// Helper function to create a default callback tuple
fn get_default_callback_tuple(metadata: PoemFunctionMetadata) -> (PoemFunctionMetadata, Box<dyn Fn(&str, Vec<String>, &mut dyn PoemFrontMatterTrait) -> anyhow::Result<()> + Send + Sync>) {
    let default_callback_fn: Box<dyn Fn(&str, Vec<String>, &mut dyn PoemFrontMatterTrait) -> anyhow::Result<()> + Send + Sync> = Box::new(|_,_,_| Ok(()));
    (metadata, default_callback_fn)
}

/// Populates the `PoemFunctionRegistry` with metadata from a `RegexConfig`.
///
/// This function iterates through each `RegexEntry` in the provided `regex_config`.
/// For each entry, it constructs a `PoemFunctionMetadata` object and inserts it
/// into the `function_registry`. The key used for insertion is the
/// `callback_function` name from the `RegexEntry`.
///
/// This ensures that all regex patterns defined in the `RegexConfig` are
/// available for use within the application's matching logic.
///
/// # Arguments
///
/// * `regex_config` - A reference to the `RegexConfig` containing the regex entries.
/// * `function_registry` - A mutable reference to the `PoemFunctionRegistry` to populate.
///
/// # Returns
///
/// A `Result` indicating success or failure.
pub fn populate_registry_from_config(
    regex_config: &RegexConfig,
    function_registry: &mut PoemFunctionRegistry,
) -> Result<()> {
    for entry in &regex_config.regexes {
        let metadata = PoemFunctionMetadata {
            regex_entry: entry.clone(), // Clone the RegexEntry
            title: None,
            summary: None,
            keywords: None,
            emojis: None,
            art_generator_instructions: None,
            pending_meme_description: None,
        };
        let callback_name = metadata.regex_entry.callback_function.clone();
        let value_to_insert: (PoemFunctionMetadata, Box<dyn Fn(&str, Vec<String>, &mut dyn PoemFrontMatterTrait) -> anyhow::Result<()> + Send + Sync>) = get_default_callback_tuple(metadata);

        // Check if the key already exists. If not, insert the new metadata and a default callback.
        if !function_registry.contains_key(&callback_name) {
            function_registry.insert(callback_name, value_to_insert);
        }
    }
    Ok(())
}
