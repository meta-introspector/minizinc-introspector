use std::collections::HashMap;
use poem_traits::{PoemFrontMatterTrait, Meme}; // Import RegexEntry and RegexConfig

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Default)]
pub struct FixedFrontMatter {
    pub title: Option<String>,
    pub summary: Option<String>,
    pub keywords: Option<Vec<String>>, // Changed from String to Vec<String>
    pub emojis: Option<String>,
    pub art_generator_instructions: Option<String>,
    pub memes: Option<Vec<Meme>>, // Changed from Vec<Meme> to Option<Vec<Meme>>
    pub poem_body: Option<String>,
    pub pending_meme_description: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct WordIndex {
    pub poems: HashMap<String, HashMap<String, usize>>,
}

/// A type alias for the function registry, mapping function names to their entries.
pub type PoemFunctionRegistry = poem_traits::FunctionRegistry;

/// A type alias for the callback function signature used in the registry.
pub type PoemCallbackFn = poem_traits::CallbackFn;

/// A type alias for a registered poem function entry, consisting of metadata and a callback.
#[allow(dead_code)]
pub type PoemFunctionEntry = poem_traits::PoemFunctionEntry;

impl PoemFrontMatterTrait for FixedFrontMatter {
    fn get_memes_mut(&mut self) -> &mut Option<Vec<Meme>> {
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
    // Removed set_keywords as it's handled by the callback directly
    fn set_emojis(&mut self, emojis: String) {
        self.emojis = Some(emojis);
    }
    fn set_art_generator_instructions(&mut self, instructions: String) {
        self.art_generator_instructions = Some(instructions);
    }
    // Removed add_meme as it's handled by the callback directly
    fn set_poem_body(&mut self, body: String) {
        self.poem_body = Some(body);
    }
    fn set_pending_meme_description(&mut self, description: String) {
        self.pending_meme_description = Some(description);
    }
}
