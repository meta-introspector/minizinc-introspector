use regex::Regex;

pub fn matches_wu(text: &str) -> bool {
    let pattern = r"^(wu|wubi|wuerstchemodelsettings|wuerstchenbuilder|wuerstcheninferencesettings|wuerstcheninner|wuerstchenmessage|wustl)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
