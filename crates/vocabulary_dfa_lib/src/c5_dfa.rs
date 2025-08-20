use regex::Regex;

pub fn matches_c5(text: &str) -> bool {
    let pattern = r"^(c56sjmy|c5fh68nj7uykauyzg2x9seq5yrvf3dkw6oojnbsc3jvo)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
