use regex::Regex;

pub fn matches_wn(text: &str) -> bool {
    let pattern = r"^(wnd_proc|wndclassw|wneboacgbsubbaak|wnew|wnwh6xopwjzk5nyu2zb3nazp|wny)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
