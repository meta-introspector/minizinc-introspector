use regex::Regex;

pub fn matches_k8(text: &str) -> bool {
    let pattern = r"^(k8s_driver|k8s_mode|k8s_openapi|k8sdriver|k8sdriverimpl|k8sdriverlogger|k8smode)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
