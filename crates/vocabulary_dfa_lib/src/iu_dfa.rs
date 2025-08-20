use regex::Regex;

pub fn matches_iu(text: &str) -> bool {
    let pattern = r"^(iu|iu1totezstwmpxafn9qi0w9msc|iuk|iukc|iukcy|ium|iuml|iunknown|iuserinfo)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
