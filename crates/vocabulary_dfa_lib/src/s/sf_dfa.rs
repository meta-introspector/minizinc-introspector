use regex::Regex;

pub fn matches_sf(text: &str) -> bool {
    let pattern = r"^(sf_ctx|sfackler|sfi|sflns4rz5a8gxke3agxcgm6usfavpxluabsrdssgy9us5eoiewq41cqdcpxbcekpksie8lyy3lnfdhevjue1wd9|sfo53|sfr|sfro|sfrow|sfrown|sft|sfx)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
