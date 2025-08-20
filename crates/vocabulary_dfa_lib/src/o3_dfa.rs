use regex::Regex;

pub fn matches_o3(text: &str) -> bool {
    let pattern = r"^(o30tyqmb16tan4wc1ztdjj|o3_mini_2025_01_31|o3_profile_config|o3_profile_overrides|o3mini)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
