use regex::Regex;

pub fn matches_ay(text: &str) -> bool {
    let pattern = r"^(ay6cooiach2vpfixnysdjnirozje5cmutguys1asxwsaksn3qdwhz3xgparwkzeumazrv|aya|ayant|aye|ayez|aygv|ayons|ayu|aywfviudxyzduxdmax03nv8tfezixa9c3sdinepju8u0v3elk6ipq6zcb|ayzb3xrze8wns6gybdsjg5v8cjyrx2zgxu2zmakcfyyd)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
