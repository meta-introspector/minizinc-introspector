use regex::Regex;

pub fn matches_hä(text: &str) -> bool {
    let pattern = r"^(häd|hän|häneen|hänelle|hänellä|häneltä|hänen|hänessä|hänestä|hänet|häntä|här)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
