use regex::Regex;

pub fn matches_hj(text: &str) -> bool {
    let pattern = r"^(hjkl|hjklˇ|hjsecpdgl|hjå)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
