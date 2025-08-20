use regex::Regex;

pub fn matches_ql(text: &str) -> bool {
    let pattern = r"^(qlmgxvswijw2c07vg|qlz2rnjiagai1hxhd9yahtjuepp)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
