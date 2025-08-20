use regex::Regex;

pub fn matches_hd(text: &str) -> bool {
    let pattern = r"^(hd6onjm2m3qzw5c8j6d1pj41mxkmzgpbsha3mykknlkagfask|hd_nvim|hdfs|hdfs_index_benchmark|hdfs_logs|hdiutil|hdl|hdln|hdr_agent_name|hdr_cert|hdr_cipher_name|hdr_metadata_flavor|hdr_ref|hdr_version|hdrhistogram|hdrop|hdrs|hdrs_slice)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
