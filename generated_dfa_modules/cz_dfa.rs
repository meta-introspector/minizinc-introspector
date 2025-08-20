use regex::Regex;

pub fn matches_cz(text: &str) -> bool {
    let pattern = r"^(cz5jthyp27kr3rwtctvjhbrgwhcurbwcyx46d8setl22|czahrrrhkx9lxf6wdcmrszklvk74c7j2vgv8vypumy6v|czstandtwofields|czvf)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
