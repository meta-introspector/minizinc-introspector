use regex::Regex;

pub fn matches_tn(text: &str) -> bool {
    let pattern = r"^(tn1|tn2|tn3|tn_map|tnhnk2ywjlbb6ihqiuhy5sobg|tnope|tnriljkpogi2sxxlgzzmqlqpmlncjmrsjz5spqyhtgg|tnsr|tnumber)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
