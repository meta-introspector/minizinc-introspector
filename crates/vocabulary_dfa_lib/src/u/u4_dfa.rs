use regex::Regex;

pub fn matches_u4(text: &str) -> bool {
    let pattern = r"^(u40|u48|u4addu0|u4addu1|u4addu2|u4addu3|u4addu4|u4addu5|u4bitandu0|u4bitandu1|u4bitandu2|u4bitandu3|u4bitandu4|u4bitandu5|u4bitoru0|u4bitoru1|u4bitoru2|u4bitoru3|u4bitoru4|u4bitoru5|u4bitxoru0|u4bitxoru1|u4bitxoru2|u4bitxoru3|u4bitxoru4|u4bitxoru5|u4cmpu0|u4cmpu1|u4cmpu2|u4cmpu3|u4cmpu4|u4cmpu5|u4divu1|u4divu2|u4divu3|u4divu4|u4divu5|u4gcdu0|u4gcdu1|u4gcdu2|u4gcdu3|u4gcdu4|u4gcdu5|u4maxu0|u4maxu1|u4maxu2|u4maxu3|u4maxu4|u4maxu5|u4minu0|u4minu1|u4minu2|u4minu3|u4minu4|u4minu5|u4mulu0|u4mulu1|u4mulu2|u4mulu3|u4mulu4|u4mulu5|u4partialdivu1|u4partialdivu2|u4partialdivu4|u4powu0|u4powu1|u4powu2|u4powu3|u4powu4|u4powu5|u4remu1|u4remu2|u4remu3|u4remu4|u4remu5|u4shlu0|u4shlu1|u4shlu2|u4shlu3|u4shlu4|u4shlu5|u4shru0|u4shru1|u4shru2|u4shru3|u4shru4|u4shru5|u4subu0|u4subu1|u4subu2|u4subu3|u4subu4)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
