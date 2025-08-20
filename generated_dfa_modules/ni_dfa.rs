use regex::Regex;

pub fn matches_ni(text: &str) -> bool {
    let pattern = r"^(ni32|nibble_mask|nibh|nicaragua|nice_wait|nicest|nicestruct|niche_optimization|niche_start|niche_tests|niche_variants|nicht|nichts|nick29581|nicksenger|nidx|niet|niets|niger|nightly1|nightly1_name|nightly2|nightly_feature_requires_nightly|nightly_feature_requires_nightly_in_dep|nightly_output|nightmare|nigra|nigripes|nii|niiden|niihin|niii|niiii|niiiii|niiiiiˇ|niiksi|niille|niillä|niiltä|niin|niinä|niissä|niistä|niitä|nijkl|nijˇk|nil_id|nile|niloticus|nilpotent|nim|nimpl_node|nimplementation|nimplication|nin_shortcut|ninclude|nincremental|nincremental_snapshot_archives_dir|nincs|nindices|nineteen|ninety_two|nineˇ|ninf|ninfo|ninitial|ninitialized|ninja|ninner|ninserted|ninsertion|ninterf_i|ninterface|ninvalid|nipper|nipple|nisd|nisi|nisl|nist|nist_algs|nistp384|nistp521|nit|nitem|nitemcfgs|nitems|nits|niv|nixpkgs|nizs759k6dnn6uyuzxc1bt3rm)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
