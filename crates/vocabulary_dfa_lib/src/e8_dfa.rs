use regex::Regex;

pub fn matches_e8(text: &str) -> bool {
    let pattern = r"^(e87zlrntk9jpkdg|e8_symmetry|e8jcgwvrvv7rwyhjthwfibeq8vah4fgneemg9aaucmaq|e8lopd1w|e8mkiwznnpgu6n55jkgzyj8ghumjchrmdfdyyfyhxwhq|e8symmetry)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
