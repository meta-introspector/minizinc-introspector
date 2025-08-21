use std::collections::HashMap;
use rand::rngs::ThreadRng;
use rand::thread_rng;

#[allow(dead_code)]
pub struct InitializedData {
    pub word_to_id: HashMap<String, usize>,
    pub id_to_word: HashMap<u32, String>, // Changed to HashMap
    pub embeddings: HashMap<u32, Vec<f64>>, // Changed to HashMap
    pub rng: ThreadRng,
}

#[allow(dead_code)]
pub fn initialize_data_structures() -> InitializedData {
    let word_to_id: HashMap<String, usize> = HashMap::new();
    let id_to_word: HashMap<u32, String> = HashMap::new(); // Changed to HashMap
    let embeddings: HashMap<u32, Vec<f64>> = HashMap::new(); // Changed to HashMap
    let rng = thread_rng();

    InitializedData {
        word_to_id,
        id_to_word,
        embeddings,
        rng,
    }
}