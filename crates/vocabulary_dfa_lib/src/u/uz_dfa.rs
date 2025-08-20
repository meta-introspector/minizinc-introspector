use regex::Regex;

pub fn matches_uz(text: &str) -> bool {
    let pattern = r"^(uz3ayetmgj1bue961fhiinqxpd0j1uu1josj|uzbek|uzdhdslw2a|uzers|uzr34)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
