use regex::Regex;

pub fn matches_kj(text: &str) -> bool {
    let pattern = r"^(kj|kj0gyhdveb1kgvqp|kjc|kjcy|kjfh76nxybbtawzhsonxfebhlyuob)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
