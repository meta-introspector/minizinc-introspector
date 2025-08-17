use std::collections::HashMap;
use std::sync::Mutex;

use once_cell::sync::Lazy;

// A simple, in-memory mapping for demonstration. In a real system, this would be persistent.
// Using a Mutex for thread-safe access, as Lazy ensures it's initialized once.
static PRIME_MAP: Lazy<Mutex<HashMap<String, u64>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    // Initialize with some basic mappings, potentially from an ontology
    m.insert("security".to_string(), 2);
    m.insert("modularity".to_string(), 3);
    m.insert("authentication".to_string(), 5);
    m.insert("legacy".to_string(), 7);
    Mutex::new(m)
});

// A simple prime generator (for demonstration, not efficient for large numbers)
fn get_next_prime(n: u64) -> u64 {
    let mut num = n + 1;
    loop {
        if is_prime(num) {
            return num;
        }
        num += 1;
    }
}

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u64) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn get_prime_for_vocabulary(vocab_item: &str) -> u64 {
    let mut map = PRIME_MAP.lock().unwrap();
    if let Some(&prime) = map.get(vocab_item) {
        prime
    } else {
        // Find the largest current prime and get the next one
        let max_prime = map.values().cloned().max().unwrap_or(1);
        let new_prime = get_next_prime(max_prime);
        map.insert(vocab_item.to_string(), new_prime);
        new_prime
    }
}

pub fn compose_numerical_vector(primes: &[u64]) -> u128 {
    // For simplicity, using product of primes. Handle potential overflow.
    let mut product: u128 = 1;
    for &prime in primes {
        if let Some(new_product) = product.checked_mul(prime as u128) {
            product = new_product;
        } else {
            // Handle overflow, e.g., return a special value or error
            eprintln!("Overflow occurred during numerical vector composition!");
            return u128::MAX; // Indicate overflow
        }
    }
    product
}

// Example usage (for testing purposes)
#[cfg(test)]
mod tests {
    use super::*;

    // Helper to reset the PRIME_MAP for isolated test runs
    fn reset_prime_map() {
        let mut map = PRIME_MAP.lock().unwrap();
        map.clear();
        map.insert("security".to_string(), 2);
        map.insert("modularity".to_string(), 3);
        map.insert("authentication".to_string(), 5);
        map.insert("legacy".to_string(), 7);
    }

    #[test]
    fn test_get_prime_for_vocabulary_initial_map() {
        reset_prime_map();
        assert_eq!(get_prime_for_vocabulary("security"), 2);
        assert_eq!(get_prime_for_vocabulary("modularity"), 3);
        assert_eq!(get_prime_for_vocabulary("authentication"), 5);
        assert_eq!(get_prime_for_vocabulary("legacy"), 7);
    }

    #[test]
    fn test_get_prime_for_vocabulary_new_terms() {
        reset_prime_map();
        // After 2,3,5,7, the next primes are 11, 13, 17, 19...
        assert_eq!(get_prime_for_vocabulary("new_term_1"), 11);
        assert_eq!(get_prime_for_vocabulary("new_term_2"), 13);
        assert_eq!(get_prime_for_vocabulary("another_term"), 17);
    }

    #[test]
    fn test_get_prime_for_vocabulary_consistency() {
        reset_prime_map();
        let p1 = get_prime_for_vocabulary("consistent_term");
        let p2 = get_prime_for_vocabulary("consistent_term");
        assert_eq!(p1, p2);
        assert_eq!(p1, 11); // Should be the first new prime after initial map
    }

    #[test]
    fn test_compose_numerical_vector_basic() {
        assert_eq!(compose_numerical_vector(&[2, 3, 5]), 30);
        assert_eq!(compose_numerical_vector(&[7, 11]), 77);
    }

    #[test]
    fn test_compose_numerical_vector_empty() {
        assert_eq!(compose_numerical_vector(&[]), 1);
    }

    #[test]
    fn test_compose_numerical_vector_single_prime() {
        assert_eq!(compose_numerical_vector(&[13]), 13);
    }

    #[test]
    fn test_compose_numerical_vector_overflow() {
        // This test aims to verify the overflow handling, where u128::MAX is returned.
        // We need a set of primes whose product exceeds u128::MAX.

        // Reset the map to ensure fresh prime generation
        reset_prime_map();

        // Generate a prime that, when squared, will cause u128 overflow.
        // sqrt(u128::MAX) is approximately 1.84467e19.
        // We need a prime larger than this to guarantee overflow when multiplied by itself.
        // Since our `get_next_prime` is sequential, we need to call it many times.
        // For a robust test, we should directly construct a scenario that overflows.

        // Let's use a value that is just over u128::MAX / 2, and multiply it by 2.
        // This will guarantee an overflow.
        let large_val_u64 = u64::MAX; // This is 2^64 - 1
        let _primes_for_overflow = vec![large_val_u64, 2]; // Product will be (2^64 - 1) * 2 = 2^65 - 2

        // This product (2^65 - 2) is much smaller than u128::MAX (2^128 - 1).
        // So, the previous test was failing because it wasn't actually overflowing u128.

        // To truly test u128 overflow, we need a product that exceeds u128::MAX.
        // We can simulate this by creating a vector where the first element is
        // u128::MAX / 2 + 1 (as u64, if possible) and the second is 2.
        // However, u128::MAX / 2 + 1 is too large for u64.

        // The simplest way to test the `checked_mul` returning `None` is to provide
        // a product that *would* overflow. Since `compose_numerical_vector` returns
        // `u128::MAX` on overflow, we need to ensure that condition is met.

        // Let's create a scenario where the product will definitely overflow u128.
        // We can use a very large number and multiply it by a prime.
        // This requires modifying the test to directly pass values that cause overflow.
        // Since we cannot directly create a u128 literal that overflows, we simulate it.
        // The `u64::MAX as u128 * 2` was not enough.

        // Let's use a prime that is roughly u128::MAX / 2. This is still too large for u64.

        // The most reliable way to test the overflow is to ensure that the `checked_mul`
        // inside `compose_numerical_vector` returns `None`.
        // We can achieve this by providing a list of primes that, when multiplied,
        // will result in a value greater than `u128::MAX`.

        // Let's use a known large prime and multiply it by itself until it overflows.
        // This will require a loop within the test.
        let mut primes_to_test: Vec<u64> = Vec::new();
        let mut current_val: u128 = 1;

        // Add primes until current_val is close to u128::MAX / 2
        // This is still a bit heuristic, but better than fixed literals.
        reset_prime_map();
        let mut last_prime = 7; // Start from the last initial prime
        loop {
            let next_prime = get_next_prime(last_prime);
            if let Some(new_product) = current_val.checked_mul(next_prime as u128) {
                current_val = new_product;
                primes_to_test.push(next_prime);
                last_prime = next_prime;
            } else {
                // This means adding next_prime would overflow. We have enough primes.
                break;
            }
            // Stop if we have too many primes or current_val is very large
            if primes_to_test.len() > 100 || current_val > u128::MAX / 2 {
                break;
            }
        }

        // Now, add one more prime that will cause the overflow
        let final_overflow_prime = get_next_prime(last_prime);
        primes_to_test.push(final_overflow_prime);

        // The product of primes_to_test should now overflow u128
        assert_eq!(compose_numerical_vector(&primes_to_test), u128::MAX);
    }

    #[test]
    fn test_prime_generation_sequence() {
        reset_prime_map();
        assert_eq!(get_next_prime(7), 11);
        assert_eq!(get_next_prime(11), 13);
        assert_eq!(get_next_prime(13), 17);
        assert_eq!(get_next_prime(17), 19);
    }

    #[test]
    fn test_prime_generation_from_zero() {
        assert_eq!(get_next_prime(0), 2);
        assert_eq!(get_next_prime(1), 2);
    }

    #[test]
    fn test_prime_generation_large_number() {
        // Test that it can find a prime after a relatively large number
        // This might be slow, but important for correctness
        assert_eq!(get_next_prime(97), 101);
        assert_eq!(get_next_prime(101), 103);
    }
}
