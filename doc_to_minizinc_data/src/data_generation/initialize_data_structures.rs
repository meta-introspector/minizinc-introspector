use std::collections::HashMap;
use rand::rngs::ThreadRng;
use rand::thread_rng;

pub struct InitializedData {
    pub word_to_id: HashMap<String, usize>,
    pub id_to_word: Vec<String>,
    pub embeddings: Vec<Vec<f64>>,
    pub rng: ThreadRng,
}

pub fn initialize_data_structures() -> InitializedData {
    let word_to_id: HashMap<String, usize> = HashMap::new();
    let id_to_word: Vec<String> = Vec::new();
    let embeddings: Vec<Vec<f64>> = Vec::new();
    let rng = thread_rng();

    InitializedData {
        word_to_id,
        id_to_word,
        embeddings,
        rng,
    }
}
