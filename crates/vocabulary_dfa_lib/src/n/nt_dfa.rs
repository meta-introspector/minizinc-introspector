use regex::Regex;

pub fn matches_nt(text: &str) -> bool {
    let pattern = r"^(nt_|nt_funcs|ntab_width|ntasks|ntconfig|ntensor|nterm|ntest_output|ntesting|ntests|ntfrc6rp0k0kkxx4sesvozutwm2x5oh3lnhcyi1xgkimf9vmjlkkm0rlo8fmj6mwf5ktbu7vs7jjpvltcrhw5tyrqvhhkuis6kbtj3gjqvrt|ntg|ntgl|nth2|nth_at_contextual_kw|nth_back_|nth_child|nth_odd|nth_prime|nth_recent_lockout|nth_set_bit|nth_set_bit_u64|nth_shape|nth_supermod|ntharg|nthchild|nthchilderror|nthchildsimple|ntheirs|nthey|nthr|nti|ntil|ntild|ntilde|ntime|ntitle|ntl|ntlg|ntln|ntoday|ntokens|ntools|ntop_level|ntotal_count_diff|ntr|ntrace|ntrait|ntransaction|ntransactions|ntri|ntria|ntrian|ntriang|ntriangl|ntriangle|ntrianglel|ntrianglele|ntrianglelef|ntriangleleft|ntrianglelefte|ntrianglelefteq|ntriangler|ntriangleri|ntrianglerig|ntrianglerigh|ntriangleright|ntrianglerighte|ntrianglerighteq|nts|ntstatus|nttt|ntttt|ntu|ntuple|ntuvwxyz|ntw|ntwelve|nty)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
