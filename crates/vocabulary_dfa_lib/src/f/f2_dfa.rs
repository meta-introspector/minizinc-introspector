use regex::Regex;

pub fn matches_f2(text: &str) -> bool {
    let pattern = r"^(f2_len|f2r1|f2r2)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
