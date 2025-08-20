use regex::Regex;

pub fn matches_p8(text: &str) -> bool {
    let pattern = r"^(p81|p8akfwqpernsztpbrwwtyzyaork74kmz56xc6nepc4j|p8p2q20ppfukakub8yvyzya24lvgwxtzhxi8zpkvkbcc)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
