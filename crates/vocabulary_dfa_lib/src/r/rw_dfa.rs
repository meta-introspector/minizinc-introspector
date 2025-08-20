use regex::Regex;

pub fn matches_rw(text: &str) -> bool {
    let pattern = r"^(rw_lock|rw_va|rwar|rwi|rwkv_v5_demo|rwkv_v6|rwkv_vocab_v20230424|rwlgt|rwlock_downgrade|rwlocked|rwlockguard_bank_hash|rwlockguard_bank_with_success_txs_hash|rwlocks|rwlocksecondaryindexentry|rworld|rwritebatch|rwriteguard|rwtxn|rwx)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
