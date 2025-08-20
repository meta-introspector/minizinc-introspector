use regex::Regex;

pub fn matches_m7(text: &str) -> bool {
    let pattern = r"^(m715|m733|m75|m78|m78g6oedteu6zotsi|m7b5)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
