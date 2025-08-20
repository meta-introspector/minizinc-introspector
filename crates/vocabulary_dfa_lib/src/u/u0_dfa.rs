use regex::Regex;

pub fn matches_u0(text: &str) -> bool {
    let pattern = r"^(u001b|u0045|u007f|u0540|u08888|u0addu0|u0addu1|u0addu2|u0addu3|u0addu4|u0addu5|u0bitandu0|u0bitandu1|u0bitandu2|u0bitandu3|u0bitandu4|u0bitandu5|u0bitoru0|u0bitoru1|u0bitoru2|u0bitoru3|u0bitoru4|u0bitoru5|u0bitxoru0|u0bitxoru1|u0bitxoru2|u0bitxoru3|u0bitxoru4|u0bitxoru5|u0cmpu0|u0cmpu1|u0cmpu2|u0cmpu3|u0cmpu4|u0cmpu5|u0divu1|u0divu2|u0divu3|u0divu4|u0divu5|u0gcdu0|u0gcdu1|u0gcdu2|u0gcdu3|u0gcdu4|u0gcdu5|u0maxu0|u0maxu1|u0maxu2|u0maxu3|u0maxu4|u0maxu5|u0minu0|u0minu1|u0minu2|u0minu3|u0minu4|u0minu5|u0mulu0|u0mulu1|u0mulu2|u0mulu3|u0mulu4|u0mulu5|u0partialdivu1|u0partialdivu2|u0partialdivu3|u0partialdivu4|u0partialdivu5|u0powu0|u0powu1|u0powu2|u0powu3|u0powu4|u0powu5|u0remu1|u0remu2|u0remu3|u0remu4|u0remu5|u0shlu0|u0shlu1|u0shlu2|u0shlu3|u0shlu4|u0shlu5|u0shru0|u0shru1|u0shru2|u0shru3|u0shru4|u0shru5|u0subu0)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
