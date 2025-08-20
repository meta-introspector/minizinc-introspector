use regex::Regex;

pub fn matches_k3(text: &str) -> bool {
    let pattern = r"^(k38brco9tdlsgz|k3ni759frv141oy4puczhv8)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
