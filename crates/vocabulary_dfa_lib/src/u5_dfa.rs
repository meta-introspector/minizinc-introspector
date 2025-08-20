use regex::Regex;

pub fn matches_u5(text: &str) -> bool {
    let pattern = r"^(u5addu0|u5addu1|u5addu2|u5addu3|u5addu4|u5addu5|u5bitandu0|u5bitandu1|u5bitandu2|u5bitandu3|u5bitandu4|u5bitandu5|u5bitoru0|u5bitoru1|u5bitoru2|u5bitoru3|u5bitoru4|u5bitoru5|u5bitxoru0|u5bitxoru1|u5bitxoru2|u5bitxoru3|u5bitxoru4|u5bitxoru5|u5cmpu0|u5cmpu1|u5cmpu2|u5cmpu3|u5cmpu4|u5cmpu5|u5divu1|u5divu2|u5divu3|u5divu4|u5divu5|u5gcdu0|u5gcdu1|u5gcdu2|u5gcdu3|u5gcdu4|u5gcdu5|u5i7tbvgz3xl2uzkc2cpylho8mghbh4t5qf3zwjlfwzoqspywbo7cn0kmp|u5maxu0|u5maxu1|u5maxu2|u5maxu3|u5maxu4|u5maxu5|u5minu0|u5minu1|u5minu2|u5minu3|u5minu4|u5minu5|u5mulu0|u5mulu1|u5mulu2|u5mulu3|u5mulu4|u5mulu5|u5partialdivu1|u5partialdivu5|u5powu0|u5powu1|u5powu2|u5powu3|u5powu4|u5powu5|u5remu1|u5remu2|u5remu3|u5remu4|u5remu5|u5shlu0|u5shlu1|u5shlu2|u5shlu3|u5shlu4|u5shlu5|u5shru0|u5shru1|u5shru2|u5shru3|u5shru4|u5shru5|u5subu0|u5subu1|u5subu2|u5subu3|u5subu4|u5subu5)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
