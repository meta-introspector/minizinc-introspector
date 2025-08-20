use regex::Regex;

pub fn matches_ah(text: &str) -> bool {
    let pattern = r"^(ahead_and_behind_upstream|ahead_by|ahead_count|ahead_num|ahead_of_remote|ahead_of_upstream|ahead_of_upstream_state|ahead_string|ahhoz|aho|ahocorasickbuilder|ahogy|ahol|ahosts)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
