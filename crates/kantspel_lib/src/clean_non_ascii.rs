pub fn clean_non_ascii(input: &str) -> String {
    input.chars().filter(|c| c.is_ascii()).collect()
}
