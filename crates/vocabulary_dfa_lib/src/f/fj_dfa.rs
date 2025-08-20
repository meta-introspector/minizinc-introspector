use regex::Regex;

pub fn matches_fj(text: &str) -> bool {
    let pattern = r"^(fj|fjc5|fjcrksbdnvijzvg|fjieivkymgzslpqob27qypukufywhrwzpcgntopzzvdh|fjl|fjli|fjlig)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
