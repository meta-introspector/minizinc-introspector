use anyhow::Result;
#[cfg(test)]
use std::collections::HashMap;
#[cfg(test)]
use regex::Regex;
#[cfg(test)]
use std::path::PathBuf; // Added PathBuf import

use poem_traits::PoemFrontMatterTrait;
#[cfg(test)]
use crate::functions::types::{
    FixedFrontMatter, PoemFunctionRegistry, PoemFunctionEntry,
    PoemCallbackFn
}; // Import from local types
#[cfg(test)]
use poem_traits::{RegexConfig, PoemFrontMatterTrait}; // Import trait from poem_traits

// Dummy callback function for testing
#[allow(dead_code)]
fn dummy_callback(
    _line: &str,
    _captures: Vec<String>,
    _fixed_fm: &mut dyn PoemFrontMatterTrait,
) -> Result<()> {
    Ok(())
}

#[test]
fn test_process_memes_with_workflow() -> Result<()> {
    let meme_lines = vec![
        "- meme: some_meme_name".to_string(),
        "- another_meme: value".to_string(),
    ];

    let mut regex_config = RegexConfig { regexes: Vec::new() };
    // Add a dummy regex entry that will match something
    regex_config.regexes.push(poem_traits::RegexEntry {
        name: "test_regex".to_string(),
        pattern: "meme: (.*)".to_string(),
        callback_function: "dummy_callback".to_string(),
    });

    let mut fixed_fm = FixedFrontMatter::default();
    let mut function_registry = PoemFunctionRegistry::new();

    // Register the dummy callback
    // Create PoemFunctionMetadata variable
    let metadata = poem_traits::PoemFunctionMetadata {
        regex_entry: poem_traits::RegexEntry {
            name: "dummy_regex".to_string(),
            pattern: "dummy_pattern".to_string(),
            callback_function: "dummy_callback".to_string(),
        },
        title: None,
        summary: None,
        keywords: None,
        emojis: None,
        art_generator_instructions: None,
        pending_meme_description: None,
    };

    // Create CallbackFn variable
    let callback_fn: poem_traits::CallbackFn = Box::new(dummy_callback);

    // Create PoemFunctionEntry tuple variable
    let poem_function_entry = (metadata, callback_fn);

    // Create &'static PoemFunctionEntry variable by leaking
    let static_poem_function_entry: &'static PoemFunctionEntry = Box::leak(Box::new(poem_function_entry));

    // Insert this 'static' reference into the function_registry
    function_registry.insert(
        "dummy_callback".to_string(),
        static_poem_function_entry,
    );

    let debug_mode = true;

    // This call should not panic or return an error if the dummy setup is correct
    let dummy_path = PathBuf::from("dummy_path.md"); // Create a dummy PathBuf
    let result = super::process_memes_with_workflow::process_memes_with_workflow(
        &dummy_path, // Added dummy path
        &meme_lines,
        &regex_config,
        &mut fixed_fm,
        &function_registry,
        debug_mode,
    );

    assert!(result.is_ok());

    Ok(())
}
