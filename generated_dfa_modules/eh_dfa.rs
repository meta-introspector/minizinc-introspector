use regex::Regex;

pub fn matches_eh(text: &str) -> bool {
    let pattern = r"^(eh45pky1kzjifb93weji91js3s4hetdsteywr7zcnpn5|eh_catch_typeinfo|ehcatchtypeinfo|ehe|ehhez|ehjj1saryxwcttc|ehlo|ehpersonality|ehv9c5vx7xqjjmpsjmzudndtzotsrwykqlzy8tvmihgj)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
