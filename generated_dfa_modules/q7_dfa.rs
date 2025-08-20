use regex::Regex;

pub fn matches_q7(text: &str) -> bool {
    let pattern = r"^(q71yuizydcgswjqnouchhatmqmivsliaff5lluorjfjkydmgelhtye_ydtbirvuomevimurkaauqlujq8xzs4lboxyf_lj2unwqyfzk1tnckbmnyrjybyl9rtbp90bexewf2wwmtu4fdbuhp2bhwmxm7eqaaaaaaaaaaaaaaaaaaaadplkd188czczqr7ewgtyipuczlzkq2lbkl3s_r|q7ohnbg)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
