use regex::Regex;

pub fn matches_tä(text: &str) -> bool {
    let pattern = r"^(tähän|täksi|tälle|tällä|tältä|tämä|tämän|tänä|tässä|tästä|tätä)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
