use regex::Regex;

pub fn matches_fp(text: &str) -> bool {
    let pattern = r"^(fp1|fp2|fp3232|fp3232_to_f32|fp32_residual_connection|fp7|fp8dot2|fp8dot4|fp8fma|fp_0|fp_00428b782a|fp_11274|fp_4b6881f2c5|fp_bytes|fp_ee1d74bde0|fp_i|fp_if_let_issue7054|fp_ignore|fp_n|fp_test|fp_ty_mantissa_nbits|fpa|fpar|fpart|fparti|fpartin|fpartint|fpi|fpn|fpr|fprint|fprint0|fps1|fps2|fptr|fpu)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
