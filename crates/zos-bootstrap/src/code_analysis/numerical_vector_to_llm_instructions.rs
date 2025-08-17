use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;

// This map should ideally be the inverse of the one in numerical_vector_generator.rs
// For now, a simple hardcoded inverse for demonstration.
static INVERSE_PRIME_MAP: Lazy<Mutex<HashMap<u64, String>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(2, "security".to_string());
    m.insert(3, "modularity".to_string());
    m.insert(5, "authentication".to_string());
    m.insert(7, "legacy".to_string());
    Mutex::new(m)
});

pub fn interpret_numerical_vector(numerical_vector: i32) -> Vec<String> {
    let mut interpreted_concepts = Vec::new();
    let mut remaining_vector = numerical_vector as u128;

    let inverse_map = INVERSE_PRIME_MAP.lock().unwrap();

    // Iterate through known primes in descending order to find factors
    let mut sorted_primes: Vec<u64> = inverse_map.keys().cloned().collect();
    sorted_primes.sort_by(|a, b| b.cmp(a)); // Sort descending

    for &prime in &sorted_primes {
        if remaining_vector % (prime as u128) == 0 {
            interpreted_concepts.push(inverse_map.get(&prime).unwrap().clone());
            remaining_vector /= prime as u128;
        }
    }

    // If there's any remaining_vector > 1, it means there are unknown primes/factors
    if remaining_vector > 1 {
        interpreted_concepts.push(format!("unknown_factor_{}", remaining_vector));
    }

    interpreted_concepts
}

pub fn generate_llm_instructions(interpreted_concepts: Vec<String>) -> String {
    if interpreted_concepts.is_empty() {
        return "Generate code based on standard practices.".to_string();
    }

    let concepts_str = interpreted_concepts.join(", ");
    format!("Generate code that strongly emphasizes the following concepts: {}.", concepts_str)
}
