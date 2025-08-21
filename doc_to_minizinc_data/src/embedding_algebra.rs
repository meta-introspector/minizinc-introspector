

#[derive(Debug, Clone)]
pub struct WordEmbedding {
    pub id: u32,
    pub word: String,
    pub embedding: Vec<f64>,
}

impl WordEmbedding {
    pub fn new(id: u32, word: String, embedding: Vec<f64>) -> Self {
        WordEmbedding { id, word, embedding }
    }

    // Basic Euclidean distance for now
    pub fn euclidean_distance(&self, other: &WordEmbedding) -> f64 {
        self.embedding.iter()
            .zip(&other.embedding)
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt()
    }

    // Placeholder for more complex algebraic operations or relationships
    // pub fn transform(&self, transformation_matrix: &[[f64; 8]; 8]) -> Self { ... }
    // pub fn combine(&self, other: &WordEmbedding) -> Self { ... }
}
