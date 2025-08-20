use regex::Regex;

pub fn matches_kx(text: &str) -> bool {
    let pattern = r"^(kx_groups|kx_padded)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
