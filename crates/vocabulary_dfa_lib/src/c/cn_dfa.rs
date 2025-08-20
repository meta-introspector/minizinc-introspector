use regex::Regex;

pub fn matches_cn(text: &str) -> bool {
    let pattern = r"^(cname|cnd6zjrtzacfvdx7psswgjtfhzuhxqfdoubqwbjgunoa|cneg|cni|cnstinf2|cnstinfb2|cnt_additional|cnt_parenthesis|cnts|cnum|cnx)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
