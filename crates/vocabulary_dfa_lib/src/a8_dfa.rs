use regex::Regex;

pub fn matches_a8(text: &str) -> bool {
    let pattern = r"^(a8_high|a8_low|a8unorm|a8xymhzovgxfkorfqemvh2pkglibip5jd7jt4zsuwo4h)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
