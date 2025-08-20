use regex::Regex;

pub fn matches_nx(text: &str) -> bool {
    let pattern = r"^(nxbbb|nxbbbb|nxccc|nxe6x7f74keprjfyckfcxbydrwkjtx1j3vsdbum9vpv|nxfff|nxggg|nxjjj|nxlll|nxmmm|nxn|nxnnn|nxooo|nxppp|nxqqq|nxrrr|nxsss|nxttt|nxuuu|nxvvv|nxwww|nxxxx|nxyyy|nxyz|nxyzf|nxzzz)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
