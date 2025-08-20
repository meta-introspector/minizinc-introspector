use regex::Regex;

pub fn matches_c0(text: &str) -> bool {
    let pattern = r"^(c0_embed|c0_logits|c0_sample|c0_symbols)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
