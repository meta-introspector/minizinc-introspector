use regex::Regex;

pub fn matches_k5(text: &str) -> bool {
    let pattern = r"^(k5dx9k8|k5my)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
