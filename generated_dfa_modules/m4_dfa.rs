use regex::Regex;

pub fn matches_m4(text: &str) -> bool {
    let pattern = r"^(m407|m420|m430|m434|m444|m453|m459|m46|m465|m476|m49|m492|m496|m4_1|m4_2|m4_3|m4b|m4m3eerctseqs8ezatrxztvkhtjydnf74kzbh58dc3yn2quxf1mexwops6l5ozbatx|m4v)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
