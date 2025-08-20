use regex::Regex;

pub fn matches_fd(text: &str) -> bool {
    let pattern = r"^(fd_double_arrow|fd_mappings|fd_path|fd_read|fd_str|fdbl|fdeclspec|fdeprecated|fdgyqdirky8nzzn9wztcztbcwlyyrxrj3lmdhqdpn5rm|fdlr|fdmapping|fdpassingfailed|fdsa|fdwrapper|fdwreason)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
