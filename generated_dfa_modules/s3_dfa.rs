use regex::Regex;

pub fn matches_s3(text: &str) -> bool {
    let pattern = r"^(s32khz|s390x|s390x_target_feature|s3_config|s3_idx|s3krit)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
