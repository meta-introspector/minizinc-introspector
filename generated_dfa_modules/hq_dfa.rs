use regex::Regex;

pub fn matches_hq(text: &str) -> bool {
    let pattern = r"^(hq|hqjtlqveggxgnyfrxuurfxv8e1swvcnsbc3456ik27hy|hqprah|hqtic8ey2qtbrwzscoacgbsubbaak)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
