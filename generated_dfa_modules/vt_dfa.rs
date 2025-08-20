use regex::Regex;

pub fn matches_vt(text: &str) -> bool {
    let pattern = r"^(vt100|vt510|vtable_address_comparisons|vtable_args|vtable_argument|vtable_construction|vtable_for|vtable_generics|vtable_impl|vtable_inner|vtable_map|vtable_mod|vtable_trait_decl|vtable_trait_impl|vtable_wm|vtabledecl|vtableentry|vtablefieldtype|vtablefieldtype_|vtablefieldvalue|vtablegetter|vtableimpl|vtencodeinfoflags|vtencodeinfoflags_|vtext|vtextplugin|vtexttokenizerparams|vtotal|vtsls_adapter|vttvdp6mhbl9j1anuky4ue1gvwnglvlohgeyrnzamgrk6|vtw3yabvt3chilxcppjorq|vtxrcvadptr|vtxsdr|vtype)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
