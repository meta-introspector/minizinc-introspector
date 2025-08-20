use regex::Regex;

pub fn matches_e5(text: &str) -> bool {
    let pattern = r"^(e500_r256|e5jifdqcwyc6qft9refympfk2mhcmv1gudysu1ue7tyv|e5nhjeoy81lvphtuaup8hnj8jjr2qhtd7)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
