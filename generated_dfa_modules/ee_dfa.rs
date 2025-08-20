use regex::Regex;

pub fn matches_ee(text: &str) -> bool {
    let pattern = r"^(ee1qdl|eecs|eees100|eees101|eel|een|eens|eents|eesnqasq9w|eex|eexist)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
