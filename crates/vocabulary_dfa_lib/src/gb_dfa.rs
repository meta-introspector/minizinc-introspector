use regex::Regex;

pub fn matches_gb(text: &str) -> bool {
    let pattern = r"^(gb1|gbindings|gbindings1|gblob|gboolean|gbp|gbr|gbre|gbrev|gbreve)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
