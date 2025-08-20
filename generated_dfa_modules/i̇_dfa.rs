use regex::Regex;

pub fn matches_i̇(text: &str) -> bool {
    let pattern = r"^(i̇|i̇g|i̇o|i̇olu|i̇st)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
