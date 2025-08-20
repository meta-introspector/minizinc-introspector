use regex::Regex;

pub fn matches_v4(text: &str) -> bool {
    let pattern = r"^(v44jd|v4_0|v4_and_git_url_encoded|v4_and_git_url_encoded_branch|v4_and_git_url_encoded_rev|v4_and_git_url_encoded_tag|v4_private_counter|v4_public_counter|v4mini)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
