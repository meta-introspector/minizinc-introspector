use regex::Regex;

pub fn matches_q1(text: &str) -> bool {
    let pattern = r"^(q104699263|q1063745|q107027826|q11900058|q12|q12345|q1751856)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
