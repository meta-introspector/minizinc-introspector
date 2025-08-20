use regex::Regex;

pub fn matches_ld(text: &str) -> bool {
    let pattern = r"^(ld2ao|ld64|ld_assume_kernel|ld_audit|ld_bind_now|ld_debug|ld_extra_libs|ld_preload|lda|ldabs|ldap|ldb|ldca|ldf|ldind|ldk|ldm|ldmia|ldo|ldq|ldqu|ldquo|ldquor|ldrd|ldrdh|ldrdha|ldrdhar|ldru|ldrus|ldrush|ldrusha|ldrushar|lds|ldsh|ldv)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
