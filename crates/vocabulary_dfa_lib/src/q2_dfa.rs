use regex::Regex;

pub fn matches_q2(text: &str) -> bool {
    let pattern = r"^(q21168966|q2587068|q2927074|q2_0|q2_1|q2_2|q2_3|q2_fixed|q2_k|q2bits|q2bytes|q2k)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
