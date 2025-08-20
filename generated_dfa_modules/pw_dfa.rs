use regex::Regex;

pub fn matches_pw(text: &str) -> bool {
    let pattern = r"^(pw1|pw2|pw_dir|pw_exp|pw_proj|pw_shell|pw_uid|pwd_path|pwr2_to_exponent|pwsh_exe)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
