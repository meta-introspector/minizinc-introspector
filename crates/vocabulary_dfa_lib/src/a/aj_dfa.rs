use regex::Regex;

pub fn matches_aj(text: &str) -> bool {
    let pattern = r"^(aj3k933zdrqhyeji2yjz8hjwxn3z3hrkjqtpte8vmunq)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
