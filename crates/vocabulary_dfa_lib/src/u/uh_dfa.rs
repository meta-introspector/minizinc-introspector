use regex::Regex;

pub fn matches_uh(text: &str) -> bool {
    let pattern = r"^(uha|uhar|uharl|uharr|uhb|uhbl|uhblk|uhlenbeck)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
