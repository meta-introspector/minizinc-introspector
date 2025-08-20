use regex::Regex;

pub fn matches_dq(text: &str) -> bool {
    let pattern = r"^(dqn|dqqgptj7pphphclzzbueyddqbyuckgrsjdsh7sp3haug|dquadsource)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
