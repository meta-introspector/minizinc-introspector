use regex::Regex;

pub fn matches_sb(text: &str) -> bool {
    let pattern = r"^(sb_h_double_arrow|sb_prelude|sb_v_double_arrow|sbf_out_path|sbf_program|sbf_sanity_list|sbf_sdk|sbf_sdk_path|sbf_to_sbf|sbf_tools_install|sbom_file_extension|sbom_files|sbom_graph|sbom_output_files|sbom_path|sbomcrate|sbomdependency|sbomdependencytype|sbomformatversion|sbomindex|sbomrustc|sboms|sbon|sbpf_version|sbpfv0_verifier_err|sbpfv1|sbpfv2|sbpfv3_return_err|sbpfv3_return_ok|sbpl|sbq|sbqu|sbquo|sbs|sbt|sbt_opts)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
