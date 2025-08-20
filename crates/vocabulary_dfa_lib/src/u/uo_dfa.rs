use regex::Regex;

pub fn matches_uo(text: &str) -> bool {
    let pattern = r"^(uo|uog|uogo|uogon|uop|uopf|uordblks)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
