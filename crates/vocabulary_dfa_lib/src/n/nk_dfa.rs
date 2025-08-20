use regex::Regex;

pub fn matches_nk(text: &str) -> bool {
    let pattern = r"^(nk2|nkeenan38|nkeep|nkk|nkkk|nklmno|nknowledge|nkqm|nkssyrfr3hda93qegogalf0n8wq244rkrsgq3v5asnedbz|nkˇk)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
