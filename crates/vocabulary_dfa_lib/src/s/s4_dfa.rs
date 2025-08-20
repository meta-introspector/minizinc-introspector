use regex::Regex;

pub fn matches_s4(text: &str) -> bool {
    let pattern = r"^(s44khz|s4dhpeauc5y)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
