use regex::Regex;

pub fn matches_nz(text: &str) -> bool {
    let pattern = r"^(nzero|nzkabuckzwx310z7x4fe4np5sy9pyia1hkrq9awq0b3v97xvw1|nzu|nzzzz)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
