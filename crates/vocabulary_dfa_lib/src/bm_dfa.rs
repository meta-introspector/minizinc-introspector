use regex::Regex;

pub fn matches_bm(text: &str) -> bool {
    let pattern = r"^(bm25_scores|bm25_weight_opt|bm25params|bm25plugin|bmap|bmaximum|bmi|bmi2|bminimum|bml|bmodel|bmp__mime|bmq|bmul|bmw)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
