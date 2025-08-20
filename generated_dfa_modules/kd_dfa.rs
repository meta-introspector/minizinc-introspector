use regex::Regex;

pub fn matches_kd(text: &str) -> bool {
    let pattern = r"^(kdd06_rp|kdf|kdim|kdir|kdtreeindex)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
