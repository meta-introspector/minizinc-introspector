use regex::Regex;

pub fn matches_h6(text: &str) -> bool {
    let pattern = r"^(h6hmvudr8xcw3euhlvfg4ecivvgo76agq1ksbl2ozods)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
