use regex::Regex;

pub fn matches_g3(text: &str) -> bool {
    let pattern = r"^(g3cj7d0leek|g3frjd9jrxcmiqchtsfvedbl2scpny3ebiuy9xxbn7a2|g3szxhhwocfunywy7efffr47rbw33ibep7b2hqndmxdu)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
