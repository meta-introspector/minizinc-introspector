use regex::Regex;

pub fn matches_ox(text: &str) -> bool {
    let pattern = r"^(ox15qc7cz|oxcart|oxide|oxigraph|oxiriref|oxygen)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
