use regex::Regex;

pub fn matches_r1(text: &str) -> bool {
    let pattern = r"^(r11|r12|r128|r13|r1389848845|r14|r15|r19|r1r0)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
