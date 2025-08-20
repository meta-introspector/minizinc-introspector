use regex::Regex;

pub fn matches_hw(text: &str) -> bool {
    let pattern = r"^(hw3sp6prebtfcnwxbnvuypmhty62gxibjfiz1zhbxfk6|hwab|hwair|hwcp|hwcqbywinir|hwidcomponent|hwnd_broadcast|hwndparent|hwndtrack|hwzeqw1yk5uilgt2uguim5ocfjncwyufbectdvpx9yub)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
