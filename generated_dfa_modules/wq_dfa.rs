use regex::Regex;

pub fn matches_wq(text: &str) -> bool {
    let pattern = r"^(wqa|wqfpgqa179cnfgwowrvruj16z6xyvxvjjwbz0wqz75xk5tksb7fnyeies4tt4jk|wqkv_size|wqmsvkrpnqmltrvhw8|wqzpe226wxh2tlfmdh9)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
