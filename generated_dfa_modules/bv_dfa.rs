use regex::Regex;

pub fn matches_bv(text: &str) -> bool {
    let pattern = r"^(bval|bvalue|bvar_case|bvec|bviously|bvra)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
