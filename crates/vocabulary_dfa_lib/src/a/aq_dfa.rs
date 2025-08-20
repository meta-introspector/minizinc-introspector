use regex::Regex;

pub fn matches_aq(text: &str) -> bool {
    let pattern = r"^(aq|aqid|aqk|aqkcagbrww|aqnsnifeqtby0fqyjujbslxs6aauqnpwunezhifnrgvj3fgmnnm7hddaguvdjs6s|aqua|aquamarine|aquatic|aquela|aquelas|aquele|aqueles|aquilo)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
