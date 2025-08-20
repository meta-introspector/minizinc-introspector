use regex::Regex;

pub fn matches_oo(text: &str) -> bool {
    let pattern = r"^(oo4np|oo7|ooan|ooanb|ooanbz|ooanbˇ|ooanbˇz|ooanfoo_and_barˇ|ooanˇ|ooce|ook|oolong|oon|ooo|oooo|oopf|oopswrongrename|oosa2cqdqutgycbomiqktm1cu4ndnebphf010gjg4if0imk1n|oov5iw6gjkqdvnli6qp0o9xkva2zmwdglcwujdkkpezlq|ooˇanb|ooˇanbz)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
