use regex::Regex;

pub fn matches_hk(text: &str) -> bool {
    let pattern = r"^(hkario|hkc|hkcu|hkern|hkey_current_user|hkey_local_machine|hkjgygttyyr2zkfjkhbn58w676fkueqxmvbtpyvrsm3n|hks|hkse|hksea|hksear|hksearo|hksearow|hksw|hkswa|hkswar|hkswaro|hkswarow|hkxy6vxdrkzocqlmdj3cyo9534fdzqxzbnwtyrjzzqjm)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
