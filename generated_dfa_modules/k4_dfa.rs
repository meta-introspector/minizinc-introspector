use regex::Regex;

pub fn matches_k4(text: &str) -> bool {
    let pattern = r"^(k4yrqkhugkwph2nlwg3q7zvrm2qvfxfugflwjq5sm7l7segom)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
