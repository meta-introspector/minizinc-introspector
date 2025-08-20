use regex::Regex;

pub fn matches_o1(text: &str) -> bool {
    let pattern = r"^(o12|o1_2024_12_17|o1_mini|o1_mini_2024_09_12|o1_preview|o1_preview_2024_09_12|o1_pro)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
