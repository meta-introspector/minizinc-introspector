use anyhow::Result;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

// Moved from poem_yaml_fixer/src/functions/types.rs
#[derive(serde::Deserialize, Debug, Clone)]
pub struct RegexEntry {
    pub name: String,
    pub pattern: String,
    pub callback_function: String,
}

// Moved from poem_yaml_fixer/src/functions/types.rs
#[derive(serde::Deserialize, Debug, Clone)]
pub struct RegexConfig {
    pub regexes: Vec<RegexEntry>,
}

// New struct to hold metadata for poem functions
#[derive(Debug, Clone)]
pub struct PoemFunctionMetadata {
    pub regex_entry: RegexEntry,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub keywords: Option<String>,
    pub emojis: Option<String>,
    pub art_generator_instructions: Option<String>,
    // pub memes: Vec<Meme>, // Memes will be processed by the function itself
    // pub poem_body: Option<String>, // Poem body is content, not metadata for the function
    pub pending_meme_description: Option<String>,
}


pub trait PoemFrontMatterTrait {
    fn get_memes_mut(&mut self) -> &mut Vec<Meme>;
    fn get_pending_meme_description_mut(&mut self) -> &mut Option<String>;

    fn set_title(&mut self, title: String);
    fn set_summary(&mut self, summary: String);
    fn set_keywords(&mut self, keywords: String);
    fn set_emojis(&mut self, emojis: String);
    fn set_art_generator_instructions(&mut self, instructions: String);
    fn add_meme(&mut self, meme: Meme);
    fn set_poem_body(&mut self, body: String);
    fn set_pending_meme_description(&mut self, description: String);
}

// Define Meme struct here, as it's part of the trait's interface
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)] // Add necessary derives
pub struct Meme {
    pub description: String,
    pub template: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub traits: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nft_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lore: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub numerology: Option<String>,
}

// Type alias for the callback function pointer
pub type CallbackFn = Box<dyn Fn(&str, Vec<String>, &mut dyn PoemFrontMatterTrait) -> Result<(), anyhow::Error> + Send + Sync + 'static>;

// Type alias for the entry in the distributed slice and function registry
pub type PoemFunctionEntry = (PoemFunctionMetadata, CallbackFn);

// Type alias for the function registry HashMap
pub type FunctionRegistry = HashMap<String, &'static PoemFunctionEntry>;
