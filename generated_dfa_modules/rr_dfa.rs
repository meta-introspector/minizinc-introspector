use regex::Regex;

pub fn matches_rr(text: &str) -> bool {
    let pattern = r"^(rra|rrange|rrangefrom|rrangeinclusive|rrangeto|rrangetoinclusive|rrar|rrarr|rrcow|rrd|rreadguard|rreceived|rrecverror|rrecvtimeouterror|rres|rresult_from|rrggbb|rrggbbaa|rrhs|rri|rrig|rrigh|rright|rrighta|rrightar|rrightarr|rrightarro|rrightarrow|rrl|rrm|rrm_with_snapshot_config|rro5v|rrrcow|rrrrarr|rrrrrslice|rrrrrstr|rrrrstring|rrrrvec|rrrstr|rrrstring|rrstr|rrstring|rrt|rrvnz)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
