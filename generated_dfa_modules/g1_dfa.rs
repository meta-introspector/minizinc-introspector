use regex::Regex;

pub fn matches_g1(text: &str) -> bool {
    let pattern = r"^(g1_compressed|g1_decompressed|g1gmfe1qum8nhdapk6bqvpgw3tqv2qc5bpkppa96qbvb|g1point)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
