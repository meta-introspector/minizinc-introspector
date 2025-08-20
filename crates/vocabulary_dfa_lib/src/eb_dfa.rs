use regex::Regex;

pub fn matches_eb(text: &str) -> bool {
    let pattern = r"^(ebahnrekk8sf88cvafaxbgkji8dv48rsp4q2sghqgwef|ebay|ebben|ebbero|ebbi|ebcdaighfe|ebeznqdjcpg8491sfskzybi5s5jtvxmpakndjmqps2kq|ebnf22rggh4zfhkja2j1ccvmjccybi74phdupef80y6nxnrxcehqzseprb6podwa|ebpfkernel|ebq48m8irrkue7znmtlvlg2uugsqhe8s8omqnmja1fjw|ebug|ebx)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
