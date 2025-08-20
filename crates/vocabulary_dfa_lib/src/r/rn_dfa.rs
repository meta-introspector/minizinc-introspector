use regex::Regex;

pub fn matches_rn(text: &str) -> bool {
    let pattern = r"^(rn_all|rname|rnc36peu75bffzyiwwpa8uzfont5swuaxjxcshooapnbufuhhsrhb2hxxz9qgniiwiwrjeshixkraaaafqchkfxo1z9h2|rnd_idx|rnewlines|rng_agent|rng_gen_usize|rnk26ocmhikmkns1z6ww|rnm|rnmi|rnmid|rnn_scan|rnn_test|rnodevec|rnodew|rnu)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
