use regex::Regex;

pub fn matches_e2(text: &str) -> bool {
    let pattern = r"^(e23sqfckyiargckkatchitsdf327ber3v4nyuq2|e2400_r224|e2big|e2e_literal_redactions)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
