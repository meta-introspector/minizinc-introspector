// This file contains the create_function_registry function.
// It creates a HashMap that maps function names (strings) to actual Rust callback functions.

use std::collections::HashMap;
//use anyhow::Result;
use once_cell::sync::Lazy;
use crate::functions::types::{
    FixedFrontMatter, Meme,
    CallbackFn};
use linkme::distributed_slice;

// Define the distributed slice where functions will register themselves
#[distributed_slice]
pub static FUNCTIONS: [&'static (String, fn() -> CallbackFn)]; // Changed type

// A global, lazily initialized HashMap for the function registry.
static FUNCTION_REGISTRY: Lazy<HashMap<String, CallbackFn>> = Lazy::new(|| {
    let mut registry = HashMap::new();
    for (name, callback_fn_ptr) in FUNCTIONS { // callback_fn_ptr is now a function pointer
        registry.insert(name.clone(), callback_fn_ptr()); // Call the function pointer to get the CallbackFn
    }
    registry
});

// This function is now just a getter for the Lazy static.
pub fn create_function_registry() -> &'static HashMap<String, CallbackFn> {
    &FUNCTION_REGISTRY
}

// Extracted callback functions with #[poem_function] attribute
//#[poem_macros::poem_function]
pub fn handle_old_meme_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut FixedFrontMatter) -> Result<()> {
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
}

//#[poem_macros::poem_function]
pub fn handle_new_meme_desc_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut FixedFrontMatter) -> Result<()> {
    fixed_fm.pending_meme_description = Some(captures[1].trim().to_string());
    Ok(())
}

//#[poem_macros::poem_function]
pub fn handle_new_meme_template_regex(_line: &str, captures: Vec<String>, fixed_fm: &mut FixedFrontMatter) -> Result<()> {
    if let Some(description) = fixed_fm.pending_meme_description.take() {
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
}
