use anyhow::Result;
//use std::collections::HashMap;
use serde::{Deserialize, Serialize};

pub trait PoemFrontMatterTrait {
    fn get_memes_mut(&mut self) -> &mut Vec<Meme>;
    fn get_pending_meme_description_mut(&mut self) -> &mut Option<String>;
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

pub type CallbackFn = Box<dyn Fn(&str, Vec<String>, &mut dyn PoemFrontMatterTrait) -> Result<(), anyhow::Error> + Send + Sync + 'static>;
