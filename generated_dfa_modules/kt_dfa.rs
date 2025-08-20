use regex::Regex;

pub fn matches_kt(text: &str) -> bool {
    let pattern = r"^(ktap6xe0b9ohi|ktexpression|kth_by|kthe|ktispropertyinputsourceid|ktispropertylocalizedname|ktispropertyunicodekeylayoutdata|ktm|ktsr9|ktulhu|kty)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
