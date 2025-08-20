use regex::Regex;

pub fn matches_g7(text: &str) -> bool {
    let pattern = r"^(g7|g74bkwbzmsbyz1kxhy44h3wjwp5hp7jbrgrudpco22ty)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
