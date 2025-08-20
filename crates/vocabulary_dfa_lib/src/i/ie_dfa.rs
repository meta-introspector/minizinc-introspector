use regex::Regex;

pub fn matches_ie(text: &str) -> bool {
    let pattern = r"^(iecy|iemand|iend|ienumexplorercommand|ienv|ier|ies|iets|iex|iexc|iexcl|iexplorercommand|iexplorercommand_impl)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
