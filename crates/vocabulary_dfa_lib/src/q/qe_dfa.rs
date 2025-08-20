use regex::Regex;

pub fn matches_qe(text: &str) -> bool {
    let pattern = r"^(qe|qed|qeury)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
