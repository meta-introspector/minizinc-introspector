use regex::Regex;

pub fn matches_v9(text: &str) -> bool {
    let pattern = r"^(v9|v9_0|v9a|v9jn|v9o8qlxzupkby5pv8zybqw)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
