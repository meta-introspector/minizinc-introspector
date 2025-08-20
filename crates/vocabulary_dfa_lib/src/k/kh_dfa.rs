use regex::Regex;

pub fn matches_kh(text: &str) -> bool {
    let pattern = r"^(kh_path|khc|khcy|khe|khmer|khrfrrdaxqem8zwxnti0ux8yt4dl5jjy0criu3xl6|khs)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
