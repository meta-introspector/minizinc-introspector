use regex::Regex;

pub fn matches_d1(text: &str) -> bool {
    let pattern = r"^(d1_in1k|d1_path|d1_t|d1dqcqi4qgzwvabi|d1h_in1k|d1s)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
