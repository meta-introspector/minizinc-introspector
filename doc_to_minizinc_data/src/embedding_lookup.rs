use std::collections::HashMap;
use crate::embedding_algebra::WordEmbedding;

pub struct EmbeddingLookup<'a> {
    id_to_word: &'a HashMap<u32, String>,
    embeddings: &'a HashMap<u32, Vec<f64>>,
    word_to_id: HashMap<String, u32>, // For efficient word to ID lookup
}

impl<'a> EmbeddingLookup<'a> {
    pub fn new(id_to_word: &'a HashMap<u32, String>, embeddings: &'a HashMap<u32, Vec<f64>>) -> Self {
        let mut word_to_id = HashMap::new();
        for (&id, word) in id_to_word.iter() {
            word_to_id.insert(word.clone(), id);
        }
        EmbeddingLookup { id_to_word, embeddings, word_to_id }
    }

    pub fn lookup_embedding(&self, word: &str) -> Option<WordEmbedding> {
        self.word_to_id.get(word).and_then(|&id| {
            self.id_to_word.get(&id).and_then(|word_str| {
                self.embeddings.get(&id).map(|embedding_vec| {
                    WordEmbedding::new(id, word_str.clone(), embedding_vec.clone())
                })
            })
        })
    }
}
