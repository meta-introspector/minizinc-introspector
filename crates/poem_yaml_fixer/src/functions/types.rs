// This file contains common data structures used across the poem_yaml_fixer crate.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
//use regex::Captures;
use anyhow::Result;

// Struct for the structured meme (same as in poem_meme_formatter)
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Meme {
    pub description: String,
    pub template: String,
    // Add any other template parameters you want here
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub traits: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nft_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lore: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub numerology: Option<String>,
}

// A simplified struct to represent the parsed front matter for fixing
// We'll build this up manually
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FixedFrontMatter {
    pub title: Option<String>,
    pub summary: Option<String>,
    pub keywords: Option<String>,
    pub emojis: Option<String>,
    pub art_generator_instructions: Option<String>,
    pub memes: Vec<Meme>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poem_body: Option<String>, // This will be extracted if found in FM
    #[serde(skip)] // Don't serialize this field
    pub pending_meme_description: Option<String>, // Temporary storage for multi-line memes
}

// New struct for the word index
#[derive(Debug, Serialize, Deserialize)]
pub struct WordIndex {
    pub poems: HashMap<String, Vec<String>>,
}

// Define structs for TOML deserialization
#[derive(Debug, Deserialize)]
pub struct RegexConfig {
    pub regexes: Vec<RegexEntry>,
}

#[derive(Debug, Deserialize)]
pub struct RegexEntry {
    pub name: String,
    pub pattern: String,
    pub callback_function: String,
}

// Define the type for our callback functions
pub type CallbackFn = Box<dyn Fn(&str, Vec<String>, &mut FixedFrontMatter) -> Result<()> + Send + Sync + 'static>;
