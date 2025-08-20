use regex::Regex;

pub fn matches_a1(text: &str) -> bool {
    let pattern = r"^(a16q37opzdqmcbe5qj6xpbb9usykfv8jzamkxvzqi4gj|a1_b2_c3|a1あ)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
