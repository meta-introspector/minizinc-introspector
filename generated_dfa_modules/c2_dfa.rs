use regex::Regex;

pub fn matches_c2(text: &str) -> bool {
    let pattern = r"^(c2_score|c2_underscore|c2na15o78qclbh4x0eqz4qcfwmgsktpc6q|c2p|c2p_att|c2p_pos|c2sp)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
