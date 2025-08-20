use regex::Regex;

pub fn matches_oz(text: &str) -> bool {
    let pattern = r"^(oz|oz3wwnrjyj95wl8ew|ozkmy|ozmkmbpwxhcp1xh3copd9yjs2lsp4ixjdakl3pfkufrwnw54jpx5fipevjeygw7ilie|ozone)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
