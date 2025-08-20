use regex::Regex;

pub fn matches_hc(text: &str) -> bool {
    let pattern = r"^(hc|hcir|hcirc|hcl|hcv5dgfjxrrj3jhdya4dceb9tedtwggyxtt3whksu2zr|hcw8zjbezyygvcbxnjwqv1t484y2556qjsfndwvjgzrh)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
