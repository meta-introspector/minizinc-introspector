use regex::Regex;

pub fn matches_m2(text: &str) -> bool {
    let pattern = r"^(m20|m21|m22|m221|m23|m232|m24|m243|m244|m245|m246|m256|m269|m27|m280|m290|m2_1|m2_2|m2_3|m2_bert_2k_retrieval_encoder_v1|m2_bert_80m_2k_retrieval|m2_bert_80m_32k_retrieval|m2r1|m2r2|m2yfnvuky)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
