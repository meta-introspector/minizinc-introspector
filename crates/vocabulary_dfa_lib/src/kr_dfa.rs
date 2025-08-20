use regex::Regex;

pub fn matches_kr(text: &str) -> bool {
    let pattern = r"^(kr|kra|kra_|krate2|krate_|krate_bytes|krate_dest|krate_download_dir|krate_file_path|krate_ident|krate_info|krate_module|krate_name|krate_req|krate_span|krateb|krd|krif5|kriz|krodher|krwgb|krähen)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
