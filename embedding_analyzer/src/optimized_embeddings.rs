use std::collections::HashMap;

#[derive(Debug)]
pub struct OptimizedEmbeddings {
    pub num_words: usize,
    pub word_map: Vec<String>,
    pub embeddings: HashMap<String, Vec<f64>>,
}
