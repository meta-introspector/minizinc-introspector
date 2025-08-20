use regex::Regex;

pub fn matches_lb(text: &str) -> bool {
    let pattern = r"^(lb1|lb2|lb_endpoints|lb_policy|lba|lback|lbar|lbarr|lbb|lbb0_1|lbb0_2|lbbr|lbbrk|lbcrypt|lbfgstype|lbfgstype1|lbfgstype2|lbk|lbpa|lbprobeports|lbr|lbra|lbrac|lbrace|lbrack|lbrk|lbrke|lbrks|lbrksl|lbrksld|lbrkslu)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
