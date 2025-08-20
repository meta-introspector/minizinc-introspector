use regex::Regex;

pub fn matches_tj(text: &str) -> bool {
    let pattern = r"^(tj6eo5urvzgaferj|tjpg|tjtcpiqd8|tjtphoe9sg4knw9xbgwecplidbqhkjo950evcnots|tjɐnu)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
