use regex::Regex;

pub fn matches_sj(text: &str) -> bool {
    let pattern = r"^(själv|sjøl)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
