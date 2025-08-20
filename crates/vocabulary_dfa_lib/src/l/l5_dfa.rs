use regex::Regex;

pub fn matches_l5(text: &str) -> bool {
    let pattern = r"^(l500|l505|l5109|l5126|l53|l531|l533|l542|l543|l55|l554|l557|l558|l57|l576|l586|l587|l5881|l599)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
