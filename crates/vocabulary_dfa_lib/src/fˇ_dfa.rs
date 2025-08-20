use regex::Regex;

pub fn matches_fˇ(text: &str) -> bool {
    let pattern = r"^(fˇile|fˇile2|fˇoo|fˇor|fˇox|fˇr|fˇur)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
