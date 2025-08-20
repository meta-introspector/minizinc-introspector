use regex::Regex;

pub fn matches_l7(text: &str) -> bool {
    let pattern = r"^(l70b|l70bchat|l717|l721|l723|l724|l726|l7270|l73|l735|l736|l743|l75|l752|l76|l766|l769c1|l772|l776|l78|l787|l790c1|l792|l793|l7b|l7bchat|l7bcode)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
