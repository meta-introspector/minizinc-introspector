use regex::Regex;

pub fn matches_s7(text: &str) -> bool {
    let pattern = r"^(s7vhbyir4knjlyhkydyh1sdubbmlhsi7mq)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
