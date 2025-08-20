use regex::Regex;

pub fn matches_a5(text: &str) -> bool {
    let pattern = r"^(a52kf8kjnvhs1y61uhkzksf82txclxzekqmfwifxlnhu)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
