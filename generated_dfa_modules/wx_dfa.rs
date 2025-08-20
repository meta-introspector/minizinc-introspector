use regex::Regex;

pub fn matches_wx(text: &str) -> bool {
    let pattern = r"^(wxayz|wxw2uxyzix)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
