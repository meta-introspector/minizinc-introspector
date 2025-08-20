use regex::Regex;

pub fn matches_b7(text: &str) -> bool {
    let pattern = r"^(b7h2caeia4zfcpe3qcgmqbiwibtwrdbrbsj1dy6ktxbq)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
