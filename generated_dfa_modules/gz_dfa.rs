use regex::Regex;

pub fn matches_gz(text: &str) -> bool {
    let pattern = r"^(gz_decoder|gz_path|gzipencoder|gzipped|gzipped_bytes|gzr4igpkeep)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
