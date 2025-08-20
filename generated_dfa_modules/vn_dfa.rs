use regex::Regex;

pub fn matches_vn(text: &str) -> bool {
    let pattern = r"^(vn_len|vn_sl|vn_start|vnc|vne|vnode|vns|vnsu|vnsub|vnsup)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
