use regex::Regex;

pub fn matches_kw(text: &str) -> bool {
    let pattern = r"^(kw_completion_in|kw_module|kw_s|kw_str|kwaak_github_token|kwaak_openai_api_key|kwaaking|kwarg|kwds|kwekking|kweuzyvcudtur7i5r|kwgq|kwin|kwnld8|kwpp299afcbwvwg1mhpsuaotsud7cv8zmjsh99aatp8x1s26yrr1)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
