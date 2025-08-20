use regex::Regex;

pub fn matches_ln(text: &str) -> bool {
    let pattern = r"^(ln0|ln_1_b|ln_1_g|ln_1p|ln_2_b|ln_2_g|ln_attn|ln_cpu|ln_f_b|ln_f_g|ln_final|ln_gpu|ln_k|ln_metal|ln_mlp|ln_out|ln_post|ln_pre|ln_text|ln_tk|ln_weight|ln_x_bias|ln_x_weight|lna|lname|lnap|lnapp|lnappr|lnappro|lnapprox|lncma7vcary6fpxuj5akqdk7ctzjnucgvzq0uufagmbaae|lncrypt|lne|lneq|lneqq|lng|lnl_cpu|lnl_gpu|lnl_metal|lnm|lnonexistinglib|lns|lnsi|lnsim|lntdll|lnx|lnxktnm)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
