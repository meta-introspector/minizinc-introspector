use regex::Regex;

pub fn matches_t1(text: &str) -> bool {
    let pattern = r"^(t100|t10k|t11|t120|t123|t13|t14|t140|t15|t160|t180|t1_actual|t1_expected|t1a|t1b|t1c)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
