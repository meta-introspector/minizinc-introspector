use std::collections::HashMap;

#[derive(Debug)]
pub struct WordData {
    pub word_to_id: HashMap<String, usize>,
    pub id_to_word: Vec<String>,
    pub embeddings: Vec<Vec<f64>>,
}

impl WordData {
    pub fn new() -> Self {
        WordData {
            word_to_id: HashMap::new(),
            id_to_word: Vec::new(),
            embeddings: Vec::new(),
        }
    }

    pub fn add_word(&mut self, word: String, embedding: Vec<f64>) -> usize {
        let id = self.id_to_word.len();
        self.word_to_id.insert(word.clone(), id);
        self.id_to_word.push(word);
        self.embeddings.push(embedding);
        id
    }

    pub fn contains_word(&self, word: &str) -> bool {
        self.word_to_id.contains_key(word)
    }

    pub fn len(&self) -> usize {
        self.id_to_word.len()
    }

    pub fn is_empty(&self) -> bool {
        self.id_to_word.is_empty()
    }
}
