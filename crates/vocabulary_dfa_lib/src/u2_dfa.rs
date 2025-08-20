use regex::Regex;

pub fn matches_u2(text: &str) -> bool {
    let pattern = r"^(u20|u2026|u243|u25|u263a|u27|u2addu0|u2addu1|u2addu2|u2addu3|u2addu4|u2addu5|u2bitandu0|u2bitandu1|u2bitandu2|u2bitandu3|u2bitandu4|u2bitandu5|u2bitoru0|u2bitoru1|u2bitoru2|u2bitoru3|u2bitoru4|u2bitoru5|u2bitxoru0|u2bitxoru1|u2bitxoru2|u2bitxoru3|u2bitxoru4|u2bitxoru5|u2cmpu0|u2cmpu1|u2cmpu2|u2cmpu3|u2cmpu4|u2cmpu5|u2divu1|u2divu2|u2divu3|u2divu4|u2divu5|u2gcdu0|u2gcdu1|u2gcdu2|u2gcdu3|u2gcdu4|u2gcdu5|u2maxu0|u2maxu1|u2maxu2|u2maxu3|u2maxu4|u2maxu5|u2minu0|u2minu1|u2minu2|u2minu3|u2minu4|u2minu5|u2muc2ljls7wdu6s9nog92vmnlngqap4irby0oqlsgdxs4j8drog|u2mulu0|u2mulu1|u2mulu2|u2mulu3|u2mulu4|u2mulu5|u2partialdivu1|u2partialdivu2|u2powu0|u2powu1|u2powu2|u2powu3|u2powu4|u2powu5|u2remu1|u2remu2|u2remu3|u2remu4|u2remu5|u2shlu0|u2shlu1|u2shlu2|u2shlu3|u2shlu4|u2shlu5|u2shru0|u2shru1|u2shru2|u2shru3|u2shru4|u2shru5|u2subu0|u2subu1|u2subu2)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
