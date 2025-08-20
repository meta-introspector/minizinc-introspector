use regex::Regex;

pub fn matches_l9(text: &str) -> bool {
    let pattern = r"^(l90|l908c30|l911|l913|l916|l919|l92|l926|l928|l932|l933|l936|l939|l947|l954|l959|l96|l969|l98|l986|l99|l991|l995)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
