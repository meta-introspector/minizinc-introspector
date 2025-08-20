use regex::Regex;

pub fn matches_nb(text: &str) -> bool {
    let pattern = r"^(nb0|nb00|nb01|nb02|nb10|nb11|nb12|nb3|nb32|nb_commas|nb_idents|nb_samples|nb_scores|nb_tokens|nbad|nbanana|nbank_snapshots_dir|nbb2|nbbb|nbbbb|nbbbbb|nbbbbbbb|nbbbbbˇ|nbc|nbcdef0123456789abcdef|nbcδf|nbe|nbegin|nbformat|nbghc|nbghi|nbig|nbip39|nbits|nbla|nbne|nbonjour|nbreak|nbringing|nbrs|nbs|nbtoy|nbu|nbuff|nbum|nbumpe|nbuttonid|nbˇ|nbˇb)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
