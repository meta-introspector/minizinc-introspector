use regex::Regex;

pub fn matches_ml(text: &str) -> bool {
    let pattern = r"^(ml318br|ml_20|ml_6|ml_8|ml_a|ml_b|ml_neg_0p5|ml_neg_1|ml_neg_20|ml_neg_px|mlabeledtr|mlc|mlcp|mld|mldr|mlfoundations|mlfrdo4sbpbpciexci3qfvslvabaj0s8wmz|mli|mlist|mlp2x_gelu|mlp_0_bias|mlp_0_weight|mlp_2_bias|mlp_2_weight|mlp_bias|mlp_depth|mlp_gelu_match|mlp_gelu_regex|mlp_layer_scale|mlp_linear1|mlp_linear2|mlp_ln|mlp_or_moe|mlp_ratios|mlp_sz|mlpembedder|mlpmaskdecoder|mlpormoe|mlpormoeblock|mlpweights|mlt|mlupipupi|mlx_dtype_str|mlx_gemm|mlx_sort|mlxfastattentionparams|mlxsort)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
