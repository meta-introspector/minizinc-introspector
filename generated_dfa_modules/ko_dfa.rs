use regex::Regex;

pub fn matches_ko(text: &str) -> bool {
    let pattern = r"^(koala|koala_13b|koala_7b|kobzol|kodama|kolors|kom|kommen|komodo|komodoensis|komondor|kon|konqueror|konsole|konst_ty|kop|kopf|kor|korleis|korso|koska|kotominearray|kould|kowudgxgq1q|kox)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
