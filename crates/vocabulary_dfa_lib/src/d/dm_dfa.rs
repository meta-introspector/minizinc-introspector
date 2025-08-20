use regex::Regex;

pub fn matches_dm(text: &str) -> bool {
    let pattern = r"^(dmcrypt|dmg|dmin|dmmv|dmxvji6bts5wczavvhiqdhqyrl61escalqpap0)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
