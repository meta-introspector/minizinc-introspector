use regex::Regex;

pub fn matches_mq(text: &str) -> bool {
    let pattern = r"^(mq|mqa_block)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
