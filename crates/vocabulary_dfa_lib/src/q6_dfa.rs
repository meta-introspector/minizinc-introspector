use regex::Regex;

pub fn matches_q6(text: &str) -> bool {
    let pattern = r"^(q607862|q6bits|q6bytes_0|q6bytes_1|q6bytes_2|q6bytes_3|q6fvv1tcjjqwkofzka2t|q6h_0|q6h_1|q6h_2|q6h_3|q6scales)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
