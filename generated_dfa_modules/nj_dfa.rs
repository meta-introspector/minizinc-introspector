use regex::Regex;

pub fn matches_nj(text: &str) -> bool {
    let pattern = r"^(nj6zx9aqa|njane|njc|njcy|njjj|njjjj|njjjjj|njson_value|njumpedoverthelaz|njumpedoverthelazydogs)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
