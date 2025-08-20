use regex::Regex;

pub fn matches_mn(text: &str) -> bool {
    let pattern = r"^(mna_fallback_with_existing_fallback|mna_fallback_with_state|mna_fallback_with_unused_state|mname|mnemonic_type|mnemosyne|mng|mnist_plot|mnistbuilder|mnk|mnode|mnopqr|mnp|mnpl|mnplu|mnplus|mnwe)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
