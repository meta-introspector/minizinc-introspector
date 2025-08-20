use regex::Regex;

pub fn matches_ly(text: &str) -> bool {
    let pattern = r"^(lycaenid|lycaon|lydian|lynx|lyon|lyra|lyrical|lysing)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
