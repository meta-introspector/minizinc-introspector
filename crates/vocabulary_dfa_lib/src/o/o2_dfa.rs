use regex::Regex;

pub fn matches_o2(text: &str) -> bool {
    let pattern = r"^(o200k_base_|o200k_base_singleton|o200kbase)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
