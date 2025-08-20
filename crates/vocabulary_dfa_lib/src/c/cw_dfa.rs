use regex::Regex;

pub fn matches_cw(text: &str) -> bool {
    let pattern = r"^(cw_customization|cw_from_custom|cw_usedefault|cwc|cwco|cwcon|cwconi|cwconin|cwconint|cwd_child|cwd_features|cwd_parts|cwd_serialized|cwd_str|cwd_temp_dir|cwd_text|cwd_to_workspace_root|cwermxme7lmbauwtzwflt6fmnpzlchaqlur2tdgfn4lq|cwi|cwin|cwint|cwitnesstree|cwmcontext|cwmimpl|cwr|cwstring)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
