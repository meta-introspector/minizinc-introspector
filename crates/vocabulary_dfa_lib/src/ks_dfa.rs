use regex::Regex;

pub fn matches_ks(text: &str) -> bool {
    let pattern = r"^(ksc|kscpropnetproxieshttpenable|kscpropnetproxieshttpport|kscpropnetproxieshttpproxy|kscpropnetproxieshttpsenable|kscpropnetproxieshttpsport|kscpropnetproxieshttpsproxy|kscr|ksecattraccount|ksecattrserver|ksecclass|ksecclassinternetpassword|ksecreturnattributes|ksecreturndata|ksecvaluedata|kseed|ksemglwow6nut6trhgabmkas5hfyq0mn3lgpzl7xjqjq6co0tzkg|ksh|kstride|ksv9io1fpiu1bpyszcdxw8ppwekkfktjhvjikajworbi4ubf7tcvkzlbzlilnqisfmvtsoqhb4hokcomjqtyjqi6enwqqhlqgage33q2zrdcnpczsviqbwhwwrh50nqt09ajl2shh)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
