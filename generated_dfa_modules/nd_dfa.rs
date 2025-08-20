use regex::Regex;

pub fn matches_nd(text: &str) -> bool {
    let pattern = r"^(nd0|nd1|nd2|nd3|nda|ndan|ndarray_a|ndarray_b|ndarray_csv|ndarrayranderror|ndarraystatsemptyerror|ndas|ndash|ndatavariant|ndb|ndbc|nddd|ndddd|nddddd|ndddddd|nddddddd|ndddddˇ|nddˇ|ndecide|ndeclaration|ndefabc|ndefault|ndefghi|ndefs|ndeletions|ndependency|ndepending|ndereferenced|nderiving|ndestination|ndetails|ndiff|ndirectory|ndisabled|ndispatching|ndjccvvse0aytndn4ph6k7svesut5ooy6pdksz9tefufjjs9hr5sjb8j1gw0gxtashhbf|ndjson|ndns|ndocs|ndone|ndream|ndrwxr|ndry|ndt|nduplicate|nduring|ndzef)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
