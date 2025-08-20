use regex::Regex;

pub fn matches_wd(text: &str) -> bool {
    let pattern = r"^(wdata|wdk|wdq|wdstogtpmrhpupjvtembe18zks9vdmoh|wdt|wdummy|wdvmq7obujgug7i5aodsb4nzpu2t3r6knpspjwdkghbg1iz7dmzhkh4apun0notc4rznlj9lbtpxu2y7yjr5w6zg68yydnofp2xtltn2jxjklyccvb39py0opitbguzzomeg0kuopojishi5jxv)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
