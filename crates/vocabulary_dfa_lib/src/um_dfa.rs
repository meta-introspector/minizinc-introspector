use regex::Regex;

pub fn matches_um(text: &str) -> bool {
    let pattern = r"^(uma|umac|umacr|umasmkzteeew3uftxdeaq|umbellus|ument|umess|umich|umios|uml_diagrams|umldiagram|umldiagrams|umm|umodulo|umount|umps|umultiply|umya_spreadsheet)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
