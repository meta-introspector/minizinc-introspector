use std::collections::HashMap;

/// Represents the Backpack Tool, demonstrating how prime numbers can be used
/// to encode letters, words, and multi-word terms, forming a basis for
/// constructing hypergraphs of meaning.
///
/// This tool illustrates the "Backpack Filling Protocol" for knowledge compression
/// and semantic embedding, where each term maps to a prime number directly or
/// through rewrite steps along a quasi fiber bundle.
pub struct BackpackTool {
    prime_map: HashMap<char, u64>,
    reverse_prime_map: HashMap<u64, char>,
}

impl BackpackTool {
    /// Creates a new BackpackTool instance with a predefined prime-to-letter mapping.
    ///
    /// This mapping is illustrative and uses the first few prime numbers.
    /// For a real-world application, a more comprehensive and robust mapping
    /// would be required.
    pub fn new() -> Self {
        let mut prime_map = HashMap::new();
        let mut reverse_prime_map = HashMap::new();

        // Simplified mapping for demonstration
        // Primes: 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
        // 83, 89, 97, 101
        let primes = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
            83, 89, 97, 101,
        ];
        let alphabet: Vec<char> = ('a'..='z').collect();

        for (i, &prime) in primes.iter().enumerate() {
            if i < alphabet.len() {
                prime_map.insert(alphabet[i], prime);
                reverse_prime_map.insert(prime, alphabet[i]);
            }
        }

        BackpackTool {
            prime_map,
            reverse_prime_map,
        }
    }

    /// Encodes a single word into a Gödel number.
    ///
    /// This is done by multiplying the prime numbers corresponding to each character.
    /// Note: This can lead to very large numbers quickly and is illustrative.
    /// For practical applications, alternative encoding schemes might be needed.
    pub fn encode_word(&self, word: &str) -> Option<u64> {
        let mut godel_number: u64 = 1;
        for char_code in word.chars() {
            if let Some(&prime) = self.prime_map.get(&char_code.to_ascii_lowercase()) {
                // Check for overflow before multiplication
                if let Some(new_godel_number) = godel_number.checked_mul(prime) {
                    godel_number = new_godel_number;
                } else {
                    // Handle overflow: return None or an error
                    return None;
                }
            } else {
                // Character not in prime map (e.g., non-alphabetic)
                return None;
            }
        }
        Some(godel_number)
    }

    /// Decodes a Gödel number back into a word (conceptual).
    ///
    /// This is a simplified conceptual decoding. True Gödel decoding involves
    /// prime factorization, which is computationally intensive for large numbers.
    pub fn decode_word(&self, godel_number: u64) -> String {
        let mut decoded_word = String::new();
        let mut temp_number = godel_number;

        // This is a highly simplified and inefficient decoding for demonstration.
        // A proper decoder would involve prime factorization.
        for (&prime, &char_val) in &self.reverse_prime_map {
            while temp_number > 0 && temp_number % prime == 0 {
                decoded_word.push(char_val);
                temp_number /= prime;
            }
        }
        // Sort the characters to get the original word order (conceptual, as prime factorization
        // doesn't preserve order directly without additional encoding).
        let mut chars: Vec<char> = decoded_word.chars().collect();
        chars.sort_by_key(|&c| self.prime_map.get(&c).unwrap_or(&0)); // Sort by prime value
        chars.into_iter().collect()
    }

    /// Encodes a multi-word term.
    ///
    /// For simplicity, this concatenates the Gödel numbers of individual words.
    /// A more robust approach might involve a higher-order Gödel numbering or hashing.
    pub fn encode_multi_word_term(&self, term: &str) -> Option<u64> {
        let words: Vec<&str> = term.split_whitespace().collect();
        let mut combined_godel_number: u64 = 1;

        for word in words {
            if let Some(word_godel) = self.encode_word(word) {
                if let Some(new_combined) = combined_godel_number.checked_mul(word_godel) {
                    combined_godel_number = new_combined;
                } else {
                    return None; // Overflow
                }
            } else {
                return None; // Word contains unmappable characters
            }
        }
        Some(combined_godel_number)
    }

    /// Illustrates the concept of layers of meaning and hypergraphs.
    ///
    /// In this conceptual model, terms (represented by their Gödel numbers) 
    /// can be nodes in a hypergraph. Relationships (hyperedges) can represent
    /// "layers of meaning" or contextual connections.
    ///
    /// For example, a hyperedge might connect:
    /// - (Term A, Term B, Context C) -> Layer of Meaning X
    /// - (Term X, Term Y, Term Z) -> Higher-order Abstraction
    ///
    /// This function simply prints a conceptual explanation.
    pub fn illustrate_hypergraph_concept(&self) {
        println!("\n--- Hypergraph of Meaning Concept ---");
        println!("In our system, terms are encoded into unique numerical representations (Gödel numbers).");
        println!("These numbers can serve as nodes in a hypergraph.");
        println!("Hyperedges represent 'layers of meaning' or complex relationships between multiple terms.");
        println!("Example: A hyperedge might connect (Term 'code', Term 'analysis', Term 'tool') to represent the concept 'Code Analysis Tool'.");
        println!("This allows us to construct a rich, multi-dimensional semantic space where relationships are not limited to binary connections.");
        println!("This aligns with the 'quasi fiber bundle' concept, where terms are points on a manifold, and relationships form the fibers.");
        println!("-------------------------------------\n");
    }
}

#[cfg(test)]
mod tests {
    use super::BackpackTool;

    #[test]
    fn test_encode_word() {
        let backpack = BackpackTool::new();
        assert_eq!(backpack.encode_word("a"), Some(2));
        assert_eq!(backpack.encode_word("b"), Some(3));
        assert_eq!(backpack.encode_word("ab"), Some(6));
        assert_eq!(backpack.encode_word("ba"), Some(6)); // Order is not preserved by simple multiplication
        assert_eq!(backpack.encode_word("rust"), Some(61 * 73 * 67 * 71)); // r=61, u=73, s=67, t=71
        assert_eq!(backpack.encode_word("hello"), Some(19 * 11 * 37 * 37 * 47)); // h=19, e=11, l=37, o=47
        assert_eq!(backpack.encode_word(""), Some(1)); // Empty string has Gödel number 1
        assert_eq!(backpack.encode_word("A"), Some(2)); // Case-insensitive
        assert_eq!(backpack.encode_word("1"), None); // Non-alphabetic character
    }

    #[test]
    fn test_decode_word() {
        let backpack = BackpackTool::new();
        // Decoding is conceptual and simplified; it won't perfectly reconstruct words
        // without additional encoding for character order.
        // This test primarily checks if the characters are present.
        let godel_number_ab = backpack.encode_word("ab").unwrap();
        let decoded_ab = backpack.decode_word(godel_number_ab);
        assert!(decoded_ab.contains('a'));
        assert!(decoded_ab.contains('b'));

        let godel_number_rust = backpack.encode_word("rust").unwrap();
        let decoded_rust = backpack.decode_word(godel_number_rust);
        assert!(decoded_rust.contains('r'));
        assert!(decoded_rust.contains('u'));
        assert!(decoded_rust.contains('s'));
        assert!(decoded_rust.contains('t'));
    }

    #[test]
    fn test_encode_multi_word_term() {
        let backpack = BackpackTool::new();
        let code_godel = backpack.encode_word("code").unwrap();
        let analysis_godel = backpack.encode_word("analysis").unwrap();
        let tool_godel = backpack.encode_word("tool").unwrap();

        let expected_combined = code_godel.checked_mul(analysis_godel).unwrap().checked_mul(tool_godel).unwrap();
        assert_eq!(backpack.encode_multi_word_term("code analysis tool"), Some(expected_combined));
        assert_eq!(backpack.encode_multi_word_term(""), Some(1));
        assert_eq!(backpack.encode_multi_word_term("hello world"), backpack.encode_word("hello").unwrap().checked_mul(backpack.encode_word("world").unwrap()));
    }

    #[test]
    fn test_overflow_handling() {
        let backpack = BackpackTool::new();
        // Create a word that will cause overflow
        let long_word = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz"; // Longer than u64 max
        assert_eq!(backpack.encode_word(long_word), None);
    }
}