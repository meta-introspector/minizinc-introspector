use regex::Regex;

pub fn matches_mr(text: &str) -> bool {
    let pattern = r"^(mr_birthday|mrdngt9qmk9dot8z7|mref|mres|mret|mrfieldaccessor|mrfunction|mritem|mritemvariant|mrkdwn|mrkpjrg79b2ok2zlgd7s3afejax9b6gaf3h9aeykrus|mrl|mrmodreflmode|mrmodule|mrnametype|mroot|mrope_sections|mrow|mrrrrarr|mrrrrrslice|mrrrrvec|mrs|mrslice|mrt|mru)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
