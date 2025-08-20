use regex::Regex;

pub fn matches_pd(text: &str) -> bool {
    let pattern = r"^(pda_|pda_seed|pda_seed_|pdataobj|pdep|pdesc|pdf2image|pdf_chunks|pdf_document_|pdf_images|pdf_reader_v1|pdf_text|pdf_tool|pdfcontent|pdfdoc|pdferror|pdfileloader|pdfloadererror|pdftext|pdl1|pdl2|pdl_at|pdl_format|pdl_image_type|pdl_parse_pdl|pdl_role|pdlschemaparseerror|pdlstruct|pdno|pdweffect|pdwfaf9uo9xalhneh8ld5bzug2bedafm2o76dhnso83mvccpnxklxu3bbx)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
