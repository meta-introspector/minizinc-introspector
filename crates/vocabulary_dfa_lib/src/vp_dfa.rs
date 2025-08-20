use regex::Regex;

pub fn matches_vp(text: &str) -> bool {
    let pattern = r"^(vp8|vpaddlq_s16|vpaddq_s16|vpath|vpc_id|vpclmulqdq|vperm_mask|vpr|vprime|vpro|vprop)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
