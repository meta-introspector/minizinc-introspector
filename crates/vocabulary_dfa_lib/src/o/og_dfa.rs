use regex::Regex;

pub fn matches_og(text: &str) -> bool {
    let pattern = r"^(ogo|ogon|ogonek|ogr|ogra|ograv|ograve|ogre|ogsxvwagxcqhqwlfg4pbnog|også|ogt)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
