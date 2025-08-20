use regex::Regex;

pub fn matches_t9(text: &str) -> bool {
    let pattern = r"^(t9n3yja7p9wjv5p2cclnuiirku5onuv)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
