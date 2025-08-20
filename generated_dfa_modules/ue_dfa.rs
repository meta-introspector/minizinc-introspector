use regex::Regex;

pub fn matches_ue(text: &str) -> bool {
    let pattern = r"^(ue|ue3tveffrp69mrgknyr71m18gdql7gxcngyyrjb3out|uee|uefa|uefi_std|uelu|uerf|uew6q|uexp)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
