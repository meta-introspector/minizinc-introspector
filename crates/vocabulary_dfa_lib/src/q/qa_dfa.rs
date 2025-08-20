use regex::Regex;

pub fn matches_qa(text: &str) -> bool {
    let pattern = r"^(qa4pa|qaaaq|qabalistic_correspondence|qabalistic_insights|qabalisticinsight|qandapredefinedmenuitem|qaqaq|qax)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
