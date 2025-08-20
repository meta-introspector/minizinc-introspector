use regex::Regex;

pub fn matches_c8(text: &str) -> bool {
    let pattern = r"^(c88ypy0ueimhxcaifkdag48jsntd65drsoqkr1ticujayrym9u1q|c8xzns1bfzait3ydexzj7g5swqwqv7tvzdncxthvnspw)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
