use regex::Regex;

pub fn matches_lq(text: &str) -> bool {
    let pattern = r"^(lq0z7bx3ccyxib0rrhowzcba8w1azvahmfnjogxcz2i|lqself|lquote)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
