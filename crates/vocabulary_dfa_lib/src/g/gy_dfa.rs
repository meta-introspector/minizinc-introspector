use regex::Regex;

pub fn matches_gy(text: &str) -> bool {
    let pattern = r"^(gyear|gyearmonth|gyitoby53e9awc56nwhj2kxmwj4do5gsmvtrowjgardw|gym|gymnasium|gyromitra|gyznzfu3w8tijjd73)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
