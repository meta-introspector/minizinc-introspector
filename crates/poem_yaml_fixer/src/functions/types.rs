use std::collections::HashMap;
use poem_traits::{PoemFrontMatterTrait, Meme, RegexEntry, RegexConfig}; // Import RegexEntry and RegexConfig

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

    fn set_title(&mut self, title: String) {
        self.title = Some(title);
    }
    fn set_summary(&mut self, summary: String) {
        self.summary = Some(summary);
    }
    fn set_keywords(&mut self, keywords: String) {
        self.keywords = Some(keywords);
    }
    fn set_emojis(&mut self, emojis: String) {
        self.emojis = Some(emojis);
    }
    fn set_art_generator_instructions(&mut self, instructions: String) {
        self.art_generator_instructions = Some(instructions);
    }
    fn add_meme(&mut self, meme: Meme) {
        self.memes.push(meme);
    }
    fn set_poem_body(&mut self, body: String) {
        self.poem_body = Some(body);
    }
    fn set_pending_meme_description(&mut self, description: String) {
        self.pending_meme_description = Some(description);
    }
}
