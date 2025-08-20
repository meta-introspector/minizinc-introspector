use regex::Regex;

pub fn matches_dv(text: &str) -> bool {
    let pattern = r"^(dv|dvaspect_content|dvb|dvector|dveuyb5m9g3ce4zpv3fxg9pcnkvh1wdsyd8xberz47jl|dvghelg8evag2o83|dvision_by_integer_zero_returns_positive_infinity|dvorak|dvse0yeehyvzopnwewggwlx11ki4a4wowmnc6guhculewe9djgejuq34lpbmdfu)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
