use regex::Regex;

pub fn matches_g6(text: &str) -> bool {
    let pattern = r"^(g6anxd6ptcsynd9znzm7j4deczajcfx7cy43obx3rkhj|g6vbf1ubok8mwb8m25ex86aoqhektzdkzuzadhkshqm6)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
