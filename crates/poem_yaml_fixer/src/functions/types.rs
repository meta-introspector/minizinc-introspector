use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use poem_traits::{PoemFrontMatterTrait, Meme}; // Import CallbackFn from poem_traits
pub use poem_traits::CallbackFn; // Import CallbackFn from poem_traits

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FixedFrontMatter {
    pub title: Option<String>,
    pub summary: Option<String>,
    pub keywords: Option<String>,
    pub emojis: Option<String>,
    pub art_generator_instructions: Option<String>,
    pub memes: Vec<Meme>,
    pub poem_body: Option<String>,
    pub pending_meme_description: Option<String>, // Used for multi-line meme descriptions
}

impl PoemFrontMatterTrait for FixedFrontMatter {
    fn get_memes_mut(&mut self) -> &mut Vec<Meme> {
        &mut self.memes
    }

    fn get_pending_meme_description_mut(&mut self) -> &mut Option<String> {
        &mut self.pending_meme_description
    }
}

// Removed: pub type CallbackFn = Box<dyn Fn(&str, Vec<String>, &mut FixedFrontMatter) -> anyhow::Result<(), anyhow::Error> + Send + Sync + 'static>;

#[derive(Debug, Deserialize, Serialize)]
pub struct RegexEntry {
    pub name: String,
    pub pattern: String,
    pub callback_function: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegexConfig {
    pub regexes: Vec<RegexEntry>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct WordIndex {
    pub poems: HashMap<String, Vec<String>>,
}
