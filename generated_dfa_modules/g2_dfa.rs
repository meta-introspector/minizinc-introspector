use regex::Regex;

pub fn matches_g2(text: &str) -> bool {
    let pattern = r"^(g2630|g2_compressed|g2_decompressed|g2point)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
