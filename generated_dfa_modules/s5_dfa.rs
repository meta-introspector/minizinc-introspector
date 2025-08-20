use regex::Regex;

pub fn matches_s5(text: &str) -> bool {
    let pattern = r"^(s5hpqs|s5s9dg5vthbsiypcidokyehba4mujq8oh5b2i71)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
