use regex::Regex;

pub fn matches_qt(text: &str) -> bool {
    let pattern = r"^(qt_im_module|qtensor_from_ggml|qterm|qterm_session_id)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
