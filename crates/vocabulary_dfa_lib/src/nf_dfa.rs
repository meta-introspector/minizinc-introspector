use regex::Regex;

pub fn matches_nf(text: &str) -> bool {
    let pattern = r"^(nfa|nfasknpb2po9dg6j1a1qfteg6sgj85h8ra|nfastembed|nfdl|nfds|nfeature|nfff|nffff|nfffff|nffffff|nfffffˇ|nffˇ|nfgh|nfghij|nfi|nfield_i|nfield_name|nfile2|nfile3|nfile_system_edits|nfileindexhigh|nfileindexlow|nfiller|nfind|nfinished|nfkd|nfl|nfo6rgemg1klbk1xunwfrqtsdxagfluwurhnry9lyndrubg7lfqzaa5obpnas9lq6ddorjqxh2lxa3psnwdksrl|nforced|nformatted|nfou|nfox|nfr|nfree|nfree1|nfree2|nfriends|nfs_super_magic|nft_cache|nft_request|nftattribute|nftrequest|nfts_minted|nfts_vec|nfull_snapshot_archives_dir|nfunction|nfuture|nfvflgi8gdvqwpyvzcncojpecwhjdmqeiyz)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
