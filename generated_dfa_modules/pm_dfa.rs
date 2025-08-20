use regex::Regex;

pub fn matches_pm(text: &str) -> bool {
    let pattern = r"^(pm_conn|pm_helper|pm_host|pm_remove|pm_target|pm_test|pm_with_int_shared|pma|pma_data|pmdep|pmuplimulip|pmypfbx6)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
