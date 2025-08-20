use regex::Regex;

pub fn matches_uq(text: &str) -> bool {
    let pattern = r"^(uqoedw9ficijaffe7pokyrrtt|uqs|uqtpec|uquh1k8lwfxsk7ehch)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
