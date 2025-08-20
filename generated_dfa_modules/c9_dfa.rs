use regex::Regex;

pub fn matches_c9(text: &str) -> bool {
    let pattern = r"^(c97ekzygrku4jxjszdjgbuy7iqr7rktr4nydwo2e5prm|c9cffpmldsqsz6wt7mrrzqunb5os4qkpjkmdaibovezz|c9oahlxdbm3sswtjx1ybgzpy55r2rarhmn1pbqn6hogh)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
