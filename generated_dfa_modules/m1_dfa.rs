use regex::Regex;

pub fn matches_m1(text: &str) -> bool {
    let pattern = r"^(m10|m100|m101|m11|m112|m12|m13|m131|m15|m16|m165|m17|m18|m192|m1920|m198|m1_1|m1_2|m1_3|m1r1|m1r2)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
