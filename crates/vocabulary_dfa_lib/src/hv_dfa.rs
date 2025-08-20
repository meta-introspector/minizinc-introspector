use regex::Regex;

pub fn matches_hv(text: &str) -> bool {
    let pattern = r"^(hv9rk|hva|hvad|hvem|hver|hvilke|hvilken|hvis|hvor|hvordan|hvorfor|hvx)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
