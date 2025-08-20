use regex::Regex;

pub fn matches_fz(text: &str) -> bool {
    let pattern = r"^(fzflkoowhzabua6rcj7ug5tvoqqaqv5njvfl4tja2i7htkffb2)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
