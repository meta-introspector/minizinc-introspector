use regex::Regex;

pub fn matches_b9(text: &str) -> bool {
    let pattern = r"^(b9cdb55u4jqsdnsdtk525ye9dmsc5ga7ybabrdfvehm9|b9erntglxoi86c6kw7ozeyddmcfd3lc3pjyx64udqcwgfo4absesmiydy43yfazh279qgh5q)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
