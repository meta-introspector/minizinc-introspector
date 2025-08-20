use regex::Regex;

pub fn matches_df(text: &str) -> bool {
    let pattern = r"^(df_size|dferxecyxy1xexcp7pt5hhq2rykhgembwtdy8efmksuzgfoov5v8ojquox0kb53x6c0tbrsukgjacm|dfferent|dfi|dfis|dfish|dfisht|dfl|dfoo|dfr|dfs_paths_via_iter|dfs_paths_via_traversal|dfsscheduler)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
