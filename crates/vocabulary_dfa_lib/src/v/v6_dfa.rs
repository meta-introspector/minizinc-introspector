use regex::Regex;

pub fn matches_v6(text: &str) -> bool {
    let pattern = r"^(v60cnjm2ipqp4b8|v6_0|v6_private_counter|v6_public_counter|v6k|v6t2)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
