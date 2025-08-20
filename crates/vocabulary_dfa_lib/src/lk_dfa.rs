use regex::Regex;

pub fn matches_lk(text: &str) -> bool {
    let pattern = r"^(lkc_use_act|lkernel32|lkfs|lkrwt0xylw5)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
