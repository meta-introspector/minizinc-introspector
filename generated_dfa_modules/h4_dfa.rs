use regex::Regex;

pub fn matches_h4(text: &str) -> bool {
    let pattern = r"^(h4siaaaaaaaca8tizcnjbwcgpha2bqaaaa)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
