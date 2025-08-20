fn main() {
    // Test words starting with 'a'
    println!("Testing words starting with 'a':");
    println!("Is 'apple' an 'a' word? {}", vocabulary_dfa_lib::a_dfa::matches_a_("apple"));
    println!("Is 'apricot' an 'a' word? {}", vocabulary_dfa_lib::a_dfa::matches_a_("apricot"));
    println!("Is 'banana' an 'a' word? {}", vocabulary_dfa_lib::a_dfa::matches_a_("banana"));

    // Test words starting with 'b'
    println!("\nTesting words starting with 'b':");
    println!("Is 'banana' a 'b' word? {}", vocabulary_dfa_lib::b__dfa::matches_b_("banana"));
    println!("Is 'blueberry' a 'b' word? {}", vocabulary_dfa_lib::b__dfa::matches_b_("blueberry"));
    println!("Is 'apple' a 'b' word? {}", vocabulary_dfa_lib::b__dfa::matches_b_("apple"));

    // You can add more tests for other letters or specific two-letter combinations
    // For example, if 'ab_dfa.rs' was generated:
    // println!("\nTesting words starting with 'ab':");
    // println!("Is 'abandon' an 'ab' word? {}", vocabulary_dfa_lib::ab_dfa::matches_ab("abandon"));
}
