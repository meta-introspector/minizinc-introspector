use regex::Regex;

pub fn matches_pv(text: &str) -> bool {
    let pattern = r"^(pv2qbqf76fo7lrdph3f542uakoopo1|pv6kjml5mp1o1hxvzqkpbdtit6gidkg3oeccixkuk0bsu9vg9vqcrmxxvgihlyoxuafyqoxv|pv_data|pva|pvec|pvh07fbv5ulfptr82ucnnjyfunsb9atd91nf5yvgu4|pvldb|pvm8s4mrb9rmxxoypslw5sguujrkuetug4wft5wmjjawqfwnbbbpbr5dkvywemy4txu6fhij4ihfvtwsmxolypjfi|pvsjyy_jgswisetzvljkb7deew5wwq1w|pvtbl)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
