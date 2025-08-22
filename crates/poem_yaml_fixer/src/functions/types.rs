use std::collections::HashMap;
use poem_traits::{PoemFrontMatterTrait, Meme};

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug, Clone)]
pub struct RegexEntry {
    pub name: String,
    pub pattern: String,
    pub callback_function: String,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug, Clone)]
pub struct RegexConfig {
    pub regexes: Vec<RegexEntry>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct FixedFrontMatter {
    pub title: Option<String>,
    pub summary: Option<String>,
    pub keywords: Option<String>,
    pub emojis: Option<String>,
    pub art_generator_instructions: Option<String>,
    pub memes: Vec<Meme>,
    pub poem_body: Option<String>,
    pub pending_meme_description: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct WordIndex {
    pub poems: HashMap<String, HashMap<String, usize>>,
}

impl PoemFrontMatterTrait for FixedFrontMatter {
    fn get_memes_mut(&mut self) -> &mut Vec<Meme> {
        &mut self.memes
    }
    fn get_pending_meme_description_mut(&mut self) -> &mut Option<String> {
        &mut self.pending_meme_description
    }
}

