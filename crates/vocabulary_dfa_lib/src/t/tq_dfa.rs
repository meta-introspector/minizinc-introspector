use regex::Regex;

pub fn matches_tq(text: &str) -> bool {
    let pattern = r"^(tq|tqdm|tqsmjxltntqfsj92x2smizjwvlg6d)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
