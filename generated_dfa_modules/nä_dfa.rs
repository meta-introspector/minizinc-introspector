use regex::Regex;

pub fn matches_nä(text: &str) -> bool {
    let pattern = r"^(näiden|näihin|näiksi|näille|näillä|näiltä|näinä|näissä|näistä|näitä|nämä|när)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
