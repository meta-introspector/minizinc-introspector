use regex::Regex;

pub fn matches_mp(text: &str) -> bool {
    let pattern = r"^(mp1e2|mp3_44100_128|mp_law|mp_tree|mpadded|mpath|mpc|mpga|mphantom|mpmc|mpmc_channel|mprescripts|mps|mpscackreceiver|mpsˇ|mptblock|mpulpilumi|mpywfnbfiz)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
