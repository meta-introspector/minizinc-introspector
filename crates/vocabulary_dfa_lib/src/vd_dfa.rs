use regex::Regex;

pub fn matches_vd(text: &str) -> bool {
    let pattern = r"^(vd_add|vd_cos|vd_div|vd_exp|vd_exp_inplace|vd_gelu|vd_ln|vd_max|vd_min|vd_mul|vd_silu|vd_sin|vd_sqr|vd_sqrt|vd_sub|vd_tanh|vd_tanh_inplace|vda|vdadd|vdas|vdash|vdashl|vdcos|vddiv|vdexp|vdf|vdf6z|vdfmax|vdfmin|vdfquhdw0ogj|vdim|vdln|vdmul|vdoc|vdot|vdot1|vdot2|vdot3|vdot4|vdot_test|vdotq_s32|vdsin|vdsp_vadd|vdsp_vaddd|vdsp_vdiv|vdsp_vdivd|vdsp_vmax|vdsp_vmaxd|vdsp_vmin|vdsp_vmind|vdsp_vmul|vdsp_vmuld|vdsp_vsub|vdsp_vsubd|vdsqrt|vdsub|vdtanh|vdupq_n_f32|vdupq_n_s32|vdupq_n_s8|vdupq_n_u8)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
