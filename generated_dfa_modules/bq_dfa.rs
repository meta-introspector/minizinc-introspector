use regex::Regex;

pub fn matches_bq(text: &str) -> bool {
    let pattern = r"^(bqdi18|bqxdzsfw8rjrhntzmsprzf77g3xgfk4hd8jyyeaffxuz)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
