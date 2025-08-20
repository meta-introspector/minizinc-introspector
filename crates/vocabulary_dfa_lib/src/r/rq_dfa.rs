use regex::Regex;

pub fn matches_rq(text: &str) -> bool {
    let pattern = r"^(rq_ctx|rqctx|rqself|rqualified|rquote)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
