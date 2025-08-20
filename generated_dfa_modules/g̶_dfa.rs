use regex::Regex;

pub fn matches_g̶(text: &str) -> bool {
    let pattern = r"^(g̶e̶n̶d̶e̶r̶s̶)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
