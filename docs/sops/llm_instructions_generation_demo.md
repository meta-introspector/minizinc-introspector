# Interpreting Solver Output and Generating LLM Instructions

## Objective
To demonstrate how the numerical output from the MiniZinc solver is interpreted back into conceptual terms and then formatted into actionable instructions for a Large Language Model (LLM). This completes the cycle of transforming code properties into numerical "vibes" and back into human-understandable guidance.

## Process Overview
As seen in `crates/zos-bootstrap/src/commands/ast_to_minizinc.rs`, the `suggested_numerical_vector` obtained from MiniZinc is passed to `numerical_vector_to_llm_instructions::interpret_numerical_vector`. The resulting interpreted concepts are then used by `numerical_vector_to_llm_instructions::generate_llm_instructions` to create the final LLM prompt.

## Interpretation Details

The relevant code for interpretation is found in `crates/zos-bootstrap/src/code_analysis/numerical_vector_to_llm_instructions.rs`:

```rust
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
```

1.  **Inverse Prime Mapping**: The `INVERSE_PRIME_MAP` provides a lookup from prime numbers back to their associated conceptual strings (e.g., 3 -> "modularity").
2.  **Factorization**: The `interpret_numerical_vector` function attempts to factorize the `suggested_numerical_vector` by iterating through known primes (from `INVERSE_PRIME_MAP`) in descending order. Each successful prime factor identifies a conceptual "vibe" that was encoded in the numerical vector.
3.  **Unknown Factors**: If, after checking all known primes, a `remaining_vector` greater than 1 exists, it indicates the presence of unknown prime factors, which are noted as `unknown_factor_X`.

## LLM Instruction Generation Details

The relevant code for generating LLM instructions is also in `crates/zos-bootstrap/src/code_analysis/numerical_vector_to_llm_instructions.rs`:

```rust
pub fn generate_llm_instructions(interpreted_concepts: Vec<String>) -> String {
    if interpreted_concepts.is_empty() {
        return "Generate code based on standard practices.".to_string();
    }

    let concepts_str = interpreted_concepts.join(", ");
    format!("Generate code that strongly emphasizes the following concepts: {}.", concepts_str)
}
```

1.  **Concept Aggregation**: The `interpreted_concepts` (a `Vec<String>`) are joined into a comma-separated string.
2.  **Instruction Formatting**: This string is then embedded into a predefined template, creating a clear instruction for the LLM.

## Example Scenario (Continuing from previous steps)

Given our example where the MiniZinc solver output `suggested_numerical_vector = 12`:

1.  **Interpretation**: `interpret_numerical_vector(12)` would factor 12.
    *   It finds 3 (modularity): `remaining_vector` becomes 4.
    *   It finds 2 (security): `remaining_vector` becomes 2.
    *   It finds 2 (security): `remaining_vector` becomes 1.
    *   The `interpreted_concepts` would be `["modularity", "security", "security"]`.
2.  **LLM Instruction**: `generate_llm_instructions(["modularity", "security", "security"])` would produce:
    `"Generate code that strongly emphasizes the following concepts: modularity, security, security."`

This final step demonstrates how the abstract numerical optimizations are translated back into concrete, actionable guidance for code generation or refactoring, effectively closing the loop of the "vibe-driven development" process.
