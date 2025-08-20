use regex::Regex;

pub fn matches_s数(text: &str) -> bool {
    let pattern = r"^(s数据表格)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
