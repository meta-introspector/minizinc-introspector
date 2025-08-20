use regex::Regex;

pub fn matches_c6(text: &str) -> bool {
    let pattern = r"^(c6ebmaxkg6jhjwkajga5yrgufg4ykxwbxf5ufv7ptexz)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
