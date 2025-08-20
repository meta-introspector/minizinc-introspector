use regex::Regex;

pub fn matches_nw(text: &str) -> bool {
    let pattern = r"^(nw_addrs|nwa|nwar|nwarh|nwarhk|nwarn|nwarps|nwarps_q4_0_ampere|nwarr|nwarro|nwarrow|nwas|nwbqjr3gpetbiavj3cbj3hfc5tmdnjdgt21hnvstvvz|nwfstyjktdsj0jqmnufh0unkivjwopquy1bum0ogo2f2oof5ypabezhudc88qsqmeu7bm9czxsgjugfzub|nwhen|nwhitespace|nwithdraw|nwn|nwne|nwnea|nwnear|nword3|nworldˇ|nworlˇd|nworˇld|nwow|nwse|nwseresize|nwwww)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
