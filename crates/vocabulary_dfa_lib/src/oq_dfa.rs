use regex::Regex;

pub fn matches_oq(text: &str) -> bool {
    let pattern = r"^(oq|oqbaa|oqe)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
