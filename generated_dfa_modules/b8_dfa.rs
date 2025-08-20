use regex::Regex;

pub fn matches_b8(text: &str) -> bool {
    let pattern = r"^(b8wvsgy99zunhdnwkrqe6lx7arvmercwugr5hwdcla9ivkmypk9pnucdpls67txwy6k9r)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
