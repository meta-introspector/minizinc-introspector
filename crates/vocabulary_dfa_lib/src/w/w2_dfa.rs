use regex::Regex;

pub fn matches_w2(text: &str) -> bool {
    let pattern = r"^(w2_0_5b|w2_1_5b|w2_72b|w2_7b|w2_b|w2_rel|w2kpoty3bsczytt1a7d3oqgm6piibgffbv)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
