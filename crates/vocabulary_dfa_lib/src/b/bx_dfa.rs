use regex::Regex;

pub fn matches_bx(text: &str) -> bool {
    let pattern = r"^(bxed_foo|bxed_str|bxed_usize|bxgf7pci64domclmpdoik4n|bxmwgfnyaqznqrcjgdseea35pcc92gftcygesj4rnfjj|bxns|bxˇbbxb|bxˇbbxˇb)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
