use regex::Regex;

pub fn matches_wv(text: &str) -> bool {
    let pattern = r"^(wvff43rwxdr1y65k1cttgkk)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
