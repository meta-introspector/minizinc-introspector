use regex::Regex;

pub fn matches_rj(text: &str) -> bool {
    let pattern = r"^(rj|rjfp0zztf8i9bsavyjfjij7vzrxcwytufwanrpzltn1qzpfh63ik92aw8avbyvea)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
