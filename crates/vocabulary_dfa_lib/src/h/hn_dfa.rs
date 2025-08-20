use regex::Regex;

pub fn matches_hn(text: &str) -> bool {
    let pattern = r"^(hnm|hnrss|hnsw_all_types|hnsw_idx|hnsw_indices|hnsw_of_type|hnsw_rs|hnsw_script|hnsw_suffix|hnufergrubb0ctyynucmkavsaghlilii0goqtsknob9as2q|hnx8ygjcqwsfgjkz6qzliepwpjpctc9ucsmd1snnqurbxv)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
