use regex::Regex;

pub fn matches_vh(text: &str) -> bool {
    let pattern = r"^(vh4torumzwirs1ouprgck4ap1qju|vhqnpkhl17e1dugzz)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
