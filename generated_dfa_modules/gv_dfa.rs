use regex::Regex;

pub fn matches_gv(text: &str) -> bool {
    let pattern = r"^(gv|gv49kkqdbnaiv2pgqhs2dy3gwyjgxmtvybykdk91orry|gvdsgdkh5gyzwpdhxnixx8vtx1kwyhh13rinapw27zxb|gve|gver|gvert|gvertn|gvertne|gvertneq|gvertneqq|gviz|gvn|gvne|gvpcitgq9dmeeojcdbivolozqc4akbudacpqpmwylwkh|gvzauzjpna1f8wmjudpb1jlvcufgzjjwsxrhmann3u0soizw8wnrfinsgpcw5e7dywf1689wcij2ye2rcy99je15fknsctzbbd04jgiyoi50mcuapcbof14vfln6bmo00cfo)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
