use regex::Regex;

pub fn matches_cˇ(text: &str) -> bool {
    let pattern = r"^(cˇ|cˇdé1|cˇo|cˇqé1)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
