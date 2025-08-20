use regex::Regex;

pub fn matches_mz(text: &str) -> bool {
    let pattern = r"^(mz|mzero|mzip|mznlib_dir_cstr)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
