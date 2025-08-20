use regex::Regex;

pub fn matches_rf(text: &str) -> bool {
    let pattern = r"^(rfc1909|rfc2388|rfc2528|rfc2822|rfc2986|rfc2988|rfc339|rfc3629|rfc4647|rfc5424|rfc5758|rfc6455|rfc6570|rfc6749|rfc7230|rfc7231|rfc7636|rfc8410|rfc8441|rfc8628|rfc9116|rfc9264|rfc_1872_exhaustive_patterns|rfc_1872_private_uninhabitedness|rfd|rfeca95xnhuwoovahuuksejlzbf7xkcluqrjoqk4zph|rfi|rfields|rfile|rfilename|rfind_event|rfis|rfish|rfisht|rfl_issue15034|rflo|rfloo|rfloor|rfn|rfn_to|rfoo|rfoo_to|rfp|rfp2acceptingstate|rfr|rframe|rfront|rfs)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
