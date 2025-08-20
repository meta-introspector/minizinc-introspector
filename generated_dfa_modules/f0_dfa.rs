use regex::Regex;

pub fn matches_f0(text: &str) -> bool {
    let pattern = r"^(f00b4r|f0_other|f0_self|f0cv)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
