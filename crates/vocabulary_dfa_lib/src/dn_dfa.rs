use regex::Regex;

pub fn matches_dn(text: &str) -> bool {
    let pattern = r"^(dna_scalar|dna_vector|dndaction|dndebug|dno|dnqualifier|dns_resolver|dns_servers|dns_tests|dnsname|dnsresolution|dnsresolve|dnt|dnt_policy|dnum)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
