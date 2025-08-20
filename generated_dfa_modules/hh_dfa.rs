use regex::Regex;

pub fn matches_hh(text: &str) -> bool {
    let pattern = r"^(hh3muyrel2bvqqa3oecaa7txju5gy6g4nxj51zvsejez|hhgahggj|hhh|hhh8ywyrv|hhhh|hhhhhhh|hhk|hhu)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
