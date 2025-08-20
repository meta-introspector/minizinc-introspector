use regex::Regex;

pub fn matches_m9(text: &str) -> bool {
    let pattern = r"^(m9|m93|m9qdwadhggawjr9awaaaabjru5erkjggg)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
