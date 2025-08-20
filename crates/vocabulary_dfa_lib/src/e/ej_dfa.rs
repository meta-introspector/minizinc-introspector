use regex::Regex;

pub fn matches_ej(text: &str) -> bool {
    let pattern = r"^(ej|eji|ejjewysddeetszhiqugnvhqhiwyzkjkfdqasd7oksagn|ejqotwvqnj4kqx5quci0ths|ejyzgbq1pmpcwxfqgme6sunhfurh1zggdqct7rv9xlzl|ejznuodjwwcbq2eangmb)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
