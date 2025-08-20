use regex::Regex;

pub fn matches_b5(text: &str) -> bool {
    let pattern = r"^(b5cklvvtqypfaur|b5qhj8sg9qpaparfcelrtlqk3goihdxirbsphhdtpgsmdvp2rps3wtngbea)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
