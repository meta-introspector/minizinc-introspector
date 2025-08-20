use regex::Regex;

pub fn matches_l6(text: &str) -> bool {
    let pattern = r"^(l60|l602|l605|l617|l6178|l618|l619|l6203|l623|l624|l625|l626|l629|l63|l630|l631|l632|l633|l634|l635|l636|l637|l638|l639|l640|l643|l644|l65|l652|l654|l658|l659|l660|l664|l67|l674|l68|l684|l685|l689|l69|l694|l6b)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
