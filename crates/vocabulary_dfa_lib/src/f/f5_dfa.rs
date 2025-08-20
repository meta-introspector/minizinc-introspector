use regex::Regex;

pub fn matches_f5(text: &str) -> bool {
    let pattern = r"^(f5_len|f5khx2crtlgp|f5x|f5yowu3hwya)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
