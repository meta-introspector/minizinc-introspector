use regex::Regex;

pub fn matches_cq(text: &str) -> bool {
    let pattern = r"^(cqdyc4et2mbfhvpgj41gxahl6exn5zopcgazshuyxwme|cqeuigilsgtfmtbsfwikaecqpm7epp7nlzfgjchvoyg5jxjiix5s0eko9ilcutllrmkw5mkhxe7bnklvbw1wdcff2|cqo_jc1p9zoecmdjwefernquqlqzsrmdlfbkdgl128zu43woliqraxaicfizspuettwmkp2d5bpwsnxs|cqqqq|cqrs|cqxdzeyrmpeio2o)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
