use regex::Regex;

pub fn matches_ct(text: &str) -> bool {
    let pattern = r"^(ct_comma|ct_eq|ct_event|ct_header|ct_lt|ct_static_equivalent|ct_static_lt|cta_text|ctarget|ctd|ctdo|ctdot|cte|ctes|ctfont|ctfontcopydefaultcascadelistforlanguages|ctfontcreatecopywithattributes|ctfontdescriptor|ctfontdescriptorcopyattribute|ctfontdescriptorcopyattributes|ctfontdescriptorcreatecopywithfeature|ctfontdescriptorcreatewithattributes|ctfontdescriptorcreatewithnameandsize|ctfontdescriptorref|ctfontref|ctheta|ctheta_prime|ctime|ctl0|ctl1|ctl_flags|ctl_term|ctl_term_i|ctline|ctlseqs|ctlz|ctlz_nonzero|ctoetc3n2|ctor_arg|ctor_arity|ctor_call_id|ctor_def_id|ctor_kind|ctor_lang_item|ctor_map|ctor_name|ctor_sub_tys|ctors_for_ty|ctpop|ctr64be|ctr_vis|ctrl_break|ctrl_c_quit_hint|ctrl_c_quit_hint_visible|ctrl_close|ctrlc_events|ctrlc_handler|ctrlc_safepoint|ctrlchandler|ctrlcmd|ctrld|ctrlshift|cts|ctsctlist|ctsx|cttz|cttz_nonzero|ctv|ctvariant|ctvhduvf8knymbyednvrrbchjjbktqwkbj6zwoqcessg|ctx0|ctx_a|ctx_b|ctx_builder|ctx_data|ctx_eval|ctx_line|ctx_message|ctx_msg|ctx_ret|ctx_signature|ctx_store|ctx_with|ctxs|cty|ctypecontextcollector)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
