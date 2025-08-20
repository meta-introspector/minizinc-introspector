use regex::Regex;

pub fn matches_bt(text: &str) -> bool {
    let pattern = r"^(bt2020cl|bt2020ncl|bt601|bt709|btail|btc|btest|bthd|bti|btls|btn_back|btn_encoded|btn_extra|btn_forward|btn_left|btn_middle|btn_right|btn_side|btree_cursors|btree_entry_insert|btree_extract_if|btree_map_retain|btree_set_entry|btree_set_range|btree_set_retain|btree_vec|btreehash|btreemap_alloc|btreemap_contains_key|btreemap_insert|btreemapentry|btreemaps|btreemapvisitor|btreeset2|btreeset_iter|btreesetasdataset|btreesetasdataset2|btreesetasgraph|bts|btvn7yjdzne6dk7ktt7ytdgmnuztngisjgsdzaetg2jf|btwmtjc8u5zlmbbuua1k6as62syjpejainat55xygdju|btype)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
