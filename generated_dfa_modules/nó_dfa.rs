use regex::Regex;

pub fn matches_nó(text: &str) -> bool {
    let pattern = r"^(nós)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
