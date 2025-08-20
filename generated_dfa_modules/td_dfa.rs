use regex::Regex;

pub fn matches_td(text: &str) -> bool {
    let pattern = r"^(td_error_icon|td_information_icon|td_opt|td_test|td_warning_icon|tdata|tdb26|tdeleting|tdep|tdi|tdo|tdocsetexclude|tdot|tds)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
