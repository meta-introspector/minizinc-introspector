use regex::Regex;

pub fn matches_fb(text: &str) -> bool {
    let pattern = r"^(fb_in1k|fb_in22k_ft_in1k|fbei6bzdbm|fbig|fboo)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
