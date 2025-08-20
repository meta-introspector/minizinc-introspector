use regex::Regex;

pub fn matches_lf(text: &str) -> bool {
    let pattern = r"^(lf2|lffacename|lfi|lfield_id|lfields|lfis|lfish|lfisht|lfl|lflo|lfloo|lfloor|lfront|lfs_dir|lft|lfu|lfwpuclnt)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
