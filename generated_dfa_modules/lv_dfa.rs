use regex::Regex;

pub fn matches_lv(text: &str) -> bool {
    let pattern = r"^(lval|lvaluereference|lvalues|lve|lver|lvert|lvertn|lvertne|lvertneq|lvertneqq|lvn|lvne)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
