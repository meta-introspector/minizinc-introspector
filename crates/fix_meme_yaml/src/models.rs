use serde::{Deserialize, Serialize};
use serde_yaml;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Meme {
    pub description: String,
    pub template: String,
    // Add any other template parameters you want here
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PoemFrontMatter {
    pub title: String,
    pub summary: String,
    pub keywords: String,
    pub emojis: String,
    pub art_generator_instructions: String,
    pub memes: Vec<Meme>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poem_body: Option<String>,
}

// Temporary struct to parse memes as raw YAML Value first
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TempPoemFrontMatter {
    pub title: String,
    pub summary: String,
    pub keywords: String,
    pub emojis: String,
    pub art_generator_instructions: String,
    pub memes: serde_yaml::Value, // Changed to serde_yaml::Value
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poem_body: Option<String>,
}
