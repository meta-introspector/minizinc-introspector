use regex::Regex;

pub fn matches_nh(text: &str) -> bool {
    let pattern = r"^(nh6eswzu3b0hwolck|nh_kd|nha|nhar|nharr|nhash|nhave|nhead|nheaders|nhhh|nhhhh|nhhhhh|nhigh|nhighest|nhij|nhijkl|nhj|nhklm|nhoping|nhovered|nhow|nhp|nhpa|nhpar|nhwpqc)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
