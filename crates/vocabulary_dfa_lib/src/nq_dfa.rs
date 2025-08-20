use regex::Regex;

pub fn matches_nq(text: &str) -> bool {
    let pattern = r"^(nq_for_hash|nqe|nqqq|nqqqq|nquads_syntax|nqualified|nquality|nquery|nquerying|nquestion|nquux|nqux)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
