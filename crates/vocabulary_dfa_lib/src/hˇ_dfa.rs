use regex::Regex;

pub fn matches_hˇ(text: &str) -> bool {
    let pattern = r"^(hˇ|hˇello|hˇi|hˇjkl|hˇllo|hˇällo|hˇäällo)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
