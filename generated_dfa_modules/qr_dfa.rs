use regex::Regex;

pub fn matches_qr(text: &str) -> bool {
    let pattern = r"^(qr_decomposition|qr_into|qr_to_svg|qrcode|qrcodeecc|qrcodegen|qrinto|qrovgtqjgnktbhs5dgwdfh6blptth15rn4buisg7umylyhqx06ckborqd33gwu|qrs|qrst|qry|qry_embed|qry_encoded)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
