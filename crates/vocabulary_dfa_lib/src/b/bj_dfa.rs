use regex::Regex;

pub fn matches_bj(text: &str) -> bool {
    let pattern = r"^(bj|bj2jmusm2irhfdlldstkhm5uqrqvqhm57hsmpibpteyu|bj3aq2ofnzyfnr1njzrjmwizzuhvfcylckh76cqsbubm|bjork|bjorn)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
