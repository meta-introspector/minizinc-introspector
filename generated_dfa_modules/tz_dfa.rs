use regex::Regex;

pub fn matches_tz(text: &str) -> bool {
    let pattern = r"^(tzatziki|tzu|tzulbdoxji2)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
