use regex::Regex;

pub fn matches_bw(text: &str) -> bool {
    let pattern = r"^(bw|bw9ua2v5igvszxboyw50|bwccbqmebgiebaaaaaucaqmmagaaacoaaaaaaaaa|bwck|bwhc|bwhk|bwqrghza2htacqq8dzp1wdahtxytywj7chxf5j7tdbae|bwugagvsbg8gdg8gd29ybgq|bwv0ysbzdhjpbmc|bwwm47plhwugjjxkqkvnirfghtpnwfnlh27na2hjqhhd)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
