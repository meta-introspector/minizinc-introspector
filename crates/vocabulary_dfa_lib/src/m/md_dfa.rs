use regex::Regex;

pub fn matches_md(text: &str) -> bool {
    let pattern = r"^(md1|md1_tokens|md2|md2withrsaencryption|md4|md4withrsaencryption|md5_|md5_hasher|md5_hasher_definition|md5_hasher_to_bytes|md5_hasher_to_bytes_definition|md5_hasher_update|md5_hasher_update_definition|md5hasher|md5withrsaencryption|md5withrsasignature|md_ce|md_editor|md_fake_servers|md_file|md_files|md_get|md_ide|md_lsp_request_count|md_reader|md_source|md_string|mda|mdas|mdb_path|mdce|mdconf|mdd|mddo|mddot|mdf|mdfind|mdhms|mdide|mdisj|mdl2|mdl_name|mdl_path|mdls|mdns_sd|mdraid|mdresult|mdrop|mds|mdt|mdt_effective_dpi|mdupont|mdvjgmr4m4rwuc8rrw24tqaik0kgnnumh70969wpsrutocmein2lxuockjkwtn3d21irjgevtjdqeqwjlhjj4pomvkb0|mdx|mdxjs)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
