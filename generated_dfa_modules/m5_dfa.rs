use regex::Regex;

pub fn matches_m5(text: &str) -> bool {
    let pattern = r"^(m500|m51|m511|m52|m56|m58|m59|m5_1|m5rvtwnwfqzioeukighjfgcgqn0plb1akaaaaaaaaaaaaaaaaaaaaaamwwy_9_odov75nonibviovmvpuqqzcw8g3p8nuud6q)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
