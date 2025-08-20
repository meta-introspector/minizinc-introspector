use regex::Regex;

pub fn matches_kl(text: &str) -> bool {
    let pattern = r"^(kl_namelength|kleene|kleeneop|klhx8ya2sfx7nhkabqlakolmaw|klm|klmnopqrst|klsiga85xw897jzvggahmkazbvhf3|klslaunchdefaults|klt|kludgy|kluv|klˇ)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
