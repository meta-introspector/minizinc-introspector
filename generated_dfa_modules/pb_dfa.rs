use regex::Regex;

pub fn matches_pb(text: &str) -> bool {
    let pattern = r"^(pb_0|pb_6|pb_8|pb_neg_3p5|pb_px|pbar|pbewithshaand128bitrc2|pbewithshaand128bitrc4|pbewithshaand2|pbewithshaand3|pbewithshaand40bitrc2|pbewithshaand40bitrc4|pbkdf2|pbm|pbm_setrange|pbm_setstep|pbm_stepit|pbn1csjkw|pbool|pbufmod|pbuttons)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
