use regex::Regex;

pub fn matches_l3(text: &str) -> bool {
    let pattern = r"^(l30|l301|l303|l304|l305|l306|l307|l30c26|l30c78|l312|l313|l315|l320|l323|l327|l3325|l336|l339|l34|l340|l347|l349|l34b|l34bcode|l354|l355|l36|l367|l378|l380|l382|l383|l388|l39|l394)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
