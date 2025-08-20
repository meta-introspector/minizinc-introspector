use regex::Regex;

pub fn matches_hx(text: &str) -> bool {
    let pattern = r"^(hxdmp|hxdyq5gixry2h6y9gqsd8kpm2jqksariohdqtlbzkrwe|hxreu1gxuh7id3puua1ohd5n4iukjyfntnxk9dvjkvgr|hxx)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
