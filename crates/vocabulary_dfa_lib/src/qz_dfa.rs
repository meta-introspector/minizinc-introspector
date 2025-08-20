use regex::Regex;

pub fn matches_qz(text: &str) -> bool {
    let pattern = r"^(qz0qq6klsfnxw9pmuoe1xba9hd1nyj14rytvj9jrfpmfuufxdb|qz7e1torh7irhyiqkcaqea7pqihffw)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
