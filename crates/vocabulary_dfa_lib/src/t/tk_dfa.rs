use regex::Regex;

pub fn matches_tk(text: &str) -> bool {
    let pattern = r"^(tkg8k4sdt9996no0jvyrnovndve2iydgrbrrrri4ilf4067xpq3igw1yfzla9kufkrwv|tkn)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
