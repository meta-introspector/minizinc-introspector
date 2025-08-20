use regex::Regex;

pub fn matches_wb(text: &str) -> bool {
    let pattern = r"^(wb_idx|wbank|wbb|wbbi|wbi|wbkkaq1wdyf1zs3gr2iiirlikkwtptuquehjschas1rrffffrrffffrrffffrrfffr96nit8b19zi6qttz9vftvxodqxhof1rs4ho6ly7veytsa6n1ghgdkk|wbr)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
