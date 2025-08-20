use regex::Regex;

pub fn matches_ok(text: &str) -> bool {
    let pattern = r"^(ok10|ok11|ok12|ok13|ok14|ok15|ok16|ok17|ok18|ok2|ok24|ok26|ok27|ok29|ok3|ok32|ok35|ok37|ok38|ok4|ok5|ok9|ok_arg|ok_arm|ok_bank_details|ok_box_trait|ok_bytes|ok_call|ok_constructor|ok_depth_1|ok_depth_2|ok_descriptor|ok_double_macro_use_shadow|ok_empty|ok_expect_info|ok_file|ok_mut|ok_name|ok_or_else_backward|ok_or_else_forward|ok_or_eyre|ok_param|ok_res|ok_result|ok_result_label|ok_return_none_ok_interrogation_backward|ok_return_none_ok_interrogation_forward|ok_shadow|ok_shadow_deep|ok_tests|ok_to_use_path_resolution|ok_token_text|ok_ty|ok_val|ok_value|ok_wrapper|okafter|okafterinside|okaliascrate|okaliasplain|okaliassuper|okbar1|okbar2|okbar3|okbaz1|okbaz2|okbaz3|okcrate|okdir|okhttpresponse|okify|okinawa|oklahoma|okmacrouse|okmacrouseinner|okmap|okplain|okresponse|okshadowstop|oksize|oksizebecausealignisusize|okv6zxn051mxdk2cisd)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
