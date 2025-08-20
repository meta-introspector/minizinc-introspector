use regex::Regex;

pub fn matches_gx(text: &str) -> bool {
    let pattern = r"^(gx3jjcs5snkladbc3t3x1joqd3fqzxg9o5uwiqlbkpfu|gxmean)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
