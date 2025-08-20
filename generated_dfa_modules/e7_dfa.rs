use regex::Regex;

pub fn matches_e7(text: &str) -> bool {
    let pattern = r"^(e7️⃣8️⃣9️⃣)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
