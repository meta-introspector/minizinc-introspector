use regex::Regex;

pub fn matches_mf(text: &str) -> bool {
    let pattern = r"^(mf16c|mfacaqeefi9cf6zxxmkhtjn1gbd7ahpbzehfoacgbsubbaaeoswdkgaeh5nxszgr|mfcyqexb3dnhxki6kjjmzck6yjmzlvpabyy2fj4nh6b|mfenced|mflags|mfma|mfpu|mfr|mfr_id|mfrac)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
