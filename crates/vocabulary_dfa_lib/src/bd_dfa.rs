use regex::Regex;

pub fn matches_bd(text: &str) -> bool {
    let pattern = r"^(bd_double_arrow|bdep_func|bdeps|bdi|bdir1|bdiv|bdo|bdptidlee8u8vknrtpguo9jqs0|bdq|bdqu|bdquo|bdyirygm4gc7uenztnzyavwq7b381ak4qdrwt51zqexkbqptunn)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
