use regex::Regex;

pub fn matches_lp(text: &str) -> bool {
    let pattern = r"^(lp9u0vngcjonevz07oeebz|lp_dist|lp_norm|lpa|lpar|lparan|lparl|lparlt|lpat|lpats|lpcreateparams|lpdist|lpfnwndproc|lplusses|lpparam|lprobs|lpszclassname|lpt1|lpt2|lpt3|lpt4|lpt5|lpt6|lpt7|lpt8|lpt9|lpvtbl)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
