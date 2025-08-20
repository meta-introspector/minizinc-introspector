use regex::Regex;

pub fn matches_fq(text: &str) -> bool {
    let pattern = r"^(fq|fq_name|fq_tool_name|fqn|fqnc7u4kohqwgrvfabjjznv8vpg6l6wwk33yjedp4yvv)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
