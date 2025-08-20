use regex::Regex;

pub fn matches_hp(text: &str) -> bool {
    let pattern = r"^(hp1|hpa|hpos_ids|hprime|hprintln|hpvmlk2kqpon|hpvy7jl)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
