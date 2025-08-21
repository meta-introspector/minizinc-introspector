use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Meme {
    pub description: String,
    pub template: String,
    pub traits: Option<Vec<String>>,
    pub nft_id: Option<String>,
    pub lore: Option<String>,
    pub numerology: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PoemDocument {
    pub title: String,
    pub summary: String,
    pub keywords: String,
    pub emojis: String,
    pub art_generator_instructions: String,
    pub memes: Vec<Meme>,
    pub poem_body: String,
}