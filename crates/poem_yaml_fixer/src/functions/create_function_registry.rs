// This file contains the create_function_registry function.
// It creates a HashMap that maps function names (strings) to actual Rust callback functions.

use std::collections::HashMap;
use anyhow::Result;
use regex::Captures;
use crate::functions::types::{FixedFrontMatter, Meme, CallbackFn}; // Import types from the types module

// Function to create the function registry
pub fn create_function_registry() -> HashMap<String, CallbackFn> {
    let mut registry: HashMap<String, CallbackFn> = HashMap::new();

    registry.insert(
        "handle_old_meme_regex".to_string(),
        Box::new(|_line, captures, fixed_fm| {
            let description = captures[1].trim().to_string();
            let template = captures[2].trim().to_string();
            fixed_fm.memes.push(Meme {
                description,
                template,
                traits: None,
                nft_id: None,
                lore: None,
                numerology: None,
            });
            Ok(())
        }),
    );

    registry.insert(
        "handle_new_meme_desc_regex".to_string(),
        Box::new(|_line, captures, fixed_fm| {
            fixed_fm.pending_meme_description = Some(captures[1].trim().to_string());
            Ok(())
        }),
    );

    registry.insert(
        "handle_new_meme_template_regex".to_string(),
        Box::new(|_line, captures, fixed_fm| {
            if let Some(description) = fixed_fm.pending_meme_description.take() { // .take() moves the value out, leaving None
                let template = captures[1].trim().to_string();
                fixed_fm.memes.push(Meme {
                    description,
                    template,
                    traits: None,
                    nft_id: None,
                    lore: None,
                    numerology: None,
                });
            } else {
                eprintln!("Warning: Template found without a preceding description for new meme format.");
            }
            Ok(())
        }),
    );

    registry
}