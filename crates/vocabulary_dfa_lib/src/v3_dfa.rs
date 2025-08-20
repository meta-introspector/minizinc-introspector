use regex::Regex;

pub fn matches_v3(text: &str) -> bool {
    let pattern = r"^(v31|v31instruct|v32_1b|v32_1binstruct|v32_3b|v32_3binstruct|v33|v3_5|v3_5large|v3_5largeturbo|v3_5medium|v3_6|v3_7|v3_70b|v3_8|v3_8b|v3_9|v3_and_git|v3_and_git_url_encoded|v3_and_git_url_encoded_branch|v3_and_git_url_encoded_rev|v3_and_git_url_encoded_tag|v3_empty|v3_index|v3_lockfile|v3empty|v3instruct)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
