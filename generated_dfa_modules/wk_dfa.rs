use regex::Regex;

pub fn matches_wk(text: &str) -> bool {
    let pattern = r"^(wkc|wkzlx346dbcrmg4emhqqih2k7hass4w2ujgpp607k119lrqeocsekkkhvj0fbfvc7cukvxdckdymcywn8rdcy2cvgbavzdrjtgtbzqb)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
