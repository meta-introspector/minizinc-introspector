use regex::Regex;

pub fn matches_cs(text: &str) -> bool {
    let pattern = r"^(cs270|cs400|cs420|cs_cpu|cs_gpu|cs_hashset|cs_hredraw|cs_iter|cs_metal|cs_offset|cs_test_1|cs_vredraw|csa|csak|csbi|csch|cscr|csd|cself|csi_sequences|csie|csky|csky_target_feature|cslot|csm1b|csmat_binop|csmatbase|csontructs|csp|cspresult|csproj|csr_options|csrc|csrf|csrf_token|csrf_token_validation_workflow|csrftoken|csroptions|css21|css_classes|css_file|css_lsp_adapter|css_mime_is_correct|css_to_core_text_font_weight|cssc|csscomputedstyleproperty|cst_kind|cstart|csteinmetz1|cstore|cstr_and_cstring_ok|cstr_bytes|cstr_cow_from_bytes|cstr_internals|cstr_type|cstring_as_c_str|cstring_from_path|cstring_literal|cstring_literals|cstrraw|cstylecastexpr|csu|csub|csube|csum|csup|csupe|csuqv42gvqljwqskyjwhqgkfharxn9hcy4yesjgaaetd|csv_content|csv_core|csv_data|csv_exporter|csv_filename|csv_path|csv_reader|csv_reader_v0|csv_result|csv_row|csv_string|csv_writer|csvec|csverror|csvfilewriter|csvisemptyerror|csvparseoptions|csvreadoptions|csvs|csvwriter|csymbol)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
