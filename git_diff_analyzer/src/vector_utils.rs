use std::collections::{HashMap, HashSet};
use ndarray::{Array1, ArrayView1};
use ndarray_linalg::Dot;

pub fn create_vocabulary(bovs: &[HashMap<String, usize>]) -> HashMap<String, usize> {
    let mut vocabulary = HashMap::new();
    let mut word_index = 0;
    for bov in bovs {
        for word in bov.keys() {
            if !vocabulary.contains_key(word) {
                vocabulary.insert(word.clone(), word_index);
                word_index += 1;
            }
        }
    }
    vocabulary
}

pub fn bov_to_vector(bov: &HashMap<String, usize>, vocabulary: &HashMap<String, usize>) -> Array1<f64> {
    let mut vector = Array1::zeros(vocabulary.len());
    for (word, count) in bov {
        if let Some(&index) = vocabulary.get(word) {
            vector[index] = *count as f64;
        }
    }
    vector
}

pub fn cosine_similarity(vec1: ArrayView1<f64>, vec2: ArrayView1<f64>) -> f64 {
    let dot_product = vec1.dot(&vec2);
    let magnitude1 = vec1.dot(&vec1).sqrt();
    let magnitude2 = vec2.dot(&vec2).sqrt();

    if magnitude1 == 0.0 || magnitude2 == 0.0 {
        0.0 // Avoid division by zero
    } else {
        dot_product / (magnitude1 * magnitude2)
    }
}
