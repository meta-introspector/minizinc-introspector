use regex::Regex;

pub fn matches_h7(text: &str) -> bool {
    let pattern = r"^(h7|h72agcpeanqdzgxkeblx4zaygpn1r9kx5wzdvh67k5qc8pgycwxuvssgiiwkaiswpua6opoetgnpb4je9zpppnnyx8ymbhghsbt7clicnaocs2a5i0iazauphhykoz1r8bwlsci9m0gpew7th1jhxk|h7okw|h7wd)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
