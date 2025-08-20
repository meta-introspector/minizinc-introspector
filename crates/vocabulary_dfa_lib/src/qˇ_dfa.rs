use regex::Regex;

pub fn matches_qˇ(text: &str) -> bool {
    let pattern = r"^(qˇ|qˇqick|qˇquick|qˇuicˇk|qˇuote)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
