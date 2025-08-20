use regex::Regex;

pub fn matches_d8(text: &str) -> bool {
    let pattern = r"^(d80qg5f6vj4yel4uu5hqttpcbfwuqoyeckbukpf8uz4|d89hyabmr2wmrtehsfkqry23wcxcdfsfnn9gmfuxhadd)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
