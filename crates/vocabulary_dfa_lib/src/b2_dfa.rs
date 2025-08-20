use regex::Regex;

pub fn matches_b2(text: &str) -> bool {
    let pattern = r"^(b2_0|b2_1|b2_2|b2_area|b2ewnwgmnd3kmpd71yzmijhml1jd4typ96zjdhmiwz7b|b2g4|b2h|b2q1|b2q2|b2yvxgr9talbrecfmvcv2b4icne4g)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
